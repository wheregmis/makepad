use crate::{route::Route, router::{Router, RouteRegistry}};
use makepad_widgets::*;

live_design! {
    pub RouterWidgetBase = {{RouterWidget}} {}
    pub RouterWidget = <RouterWidgetBase> {
        width: Fill, height: Fill
    }
}

/// Router widget for managing navigation between pages
#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct RouterWidget {
    #[rust]
    area: Area,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    active_route: LiveId,
    #[live]
    default_route: LiveId,
    #[live(false)]
    persist_state: bool,
    #[rust]
    router: Router,
    #[rust]
    route_templates: ComponentMap<LiveId, LivePtr>,
    #[rust]
    route_widgets: ComponentMap<LiveId, WidgetRef>,
    #[rust]
    child_routers: ComponentMap<LiveId, RouterWidgetRef>,
    #[rust]
    route_patterns: ComponentMap<LiveId, String>,
    #[rust]
    route_registry: RouteRegistry,
    #[rust]
    route_change_callbacks: Vec<Box<dyn Fn(&mut Cx, Option<Route>, Route) + Send + Sync>>,
}

impl LiveHook for RouterWidget {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        apply: &mut Apply,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
        if let ApplyFrom::UpdateFromDoc { .. } = apply.from {
            self.route_templates.clear();
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        match apply.from {
            ApplyFrom::NewFromDoc { .. } | ApplyFrom::UpdateFromDoc { .. } => {
                if self.router.current_route().is_none() {
                    let initial_route = if self.active_route.0 != 0 {
                        self.active_route
                    } else {
                        self.default_route
                    };

                    if initial_route.0 != 0 {
                        self.router = if self.persist_state {
                            Router::with_persistence(Route::new(initial_route))
                        } else {
                            Router::new(Route::new(initial_route))
                        };
                        self.active_route = initial_route;
                    }
                }

                // Create widgets for ALL routes, not just the active one
                // This ensures buttons on inactive pages can still generate events
                for (route_id, _ptr) in self.route_templates.iter() {
                    if !self.route_widgets.contains_key(route_id) {
                        if let Some(ptr) = self.route_templates.get(route_id) {
                            self.route_widgets.get_or_insert(cx, *route_id, |cx| {
                                let mut widget = WidgetRef::empty();
                                cx.get_nodes_from_live_ptr(*ptr, |cx, file_id, index, nodes| {
                                    let route_pattern_idx = nodes.child_by_name(
                                        index,
                                        LiveProp(live_id!(route_pattern), LivePropType::Field),
                                    );
                                    apply.override_from(ApplyFrom::NewFromDoc { file_id }, |apply| {
                                        Self::apply_widget_silencing_route_pattern(
                                            cx,
                                            apply,
                                            index,
                                            nodes,
                                            &mut widget,
                                            route_pattern_idx,
                                        );
                                    });
                                    nodes.skip_node(index)
                                });
                                widget
                            });
                        }
                    }
                }

                // Auto-detect child routers in route widgets
                self.detect_child_routers(cx);
            }
            _ => (),
        }
    }

    fn apply_value_instance(
        &mut self,
        cx: &mut Cx,
        apply: &mut Apply,
        index: usize,
        nodes: &[LiveNode],
    ) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc { file_id } | ApplyFrom::UpdateFromDoc { file_id, .. } => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    let live_ptr = cx
                        .live_registry
                        .borrow()
                        .file_id_index_to_live_ptr(file_id, index);
                    self.route_templates.insert(id, live_ptr);

                    // Scan for route_pattern property in child nodes and register it
                    if let Some(pattern_node_idx) = nodes.child_by_name(index, LiveProp(live_id!(route_pattern), LivePropType::Field)) {
                        let pattern_node = &nodes[pattern_node_idx];
                        if let LiveValue::Str(pattern) = &pattern_node.value {
                            let pattern_str = pattern.to_string();
                            self.route_patterns.insert(id, pattern_str.clone());
                            // Auto-register the pattern
                            if let Err(e) = self.route_registry.register_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            }
                        } else if let LiveValue::String(pattern) = &pattern_node.value {
                            let pattern_str = pattern.as_str().to_string();
                            self.route_patterns.insert(id, pattern_str.clone());
                            // Auto-register the pattern
                            if let Err(e) = self.route_registry.register_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            }
                        }
                    }

                    // Create/update the route widget instance. We silence `route_pattern` by marking
                    // it as a prefixed property before applying, so it is ignored by the default
                    // `apply_value_unknown` handler (no noisy "no matching field" warning).
                    let route_pattern_idx =
                        nodes.child_by_name(index, LiveProp(live_id!(route_pattern), LivePropType::Field));

                    let widget = self
                        .route_widgets
                        .get_or_insert(cx, id, |_cx| WidgetRef::empty());

                    Self::apply_widget_silencing_route_pattern(cx, apply, index, nodes, widget, route_pattern_idx);
                } else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => (),
        }
        nodes.skip_node(index)
    }
}

impl RouterWidget {
    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if self.route_templates.contains_key(&route_id) {
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            if let Some(ptr) = self.route_templates.get(&route_id) {
                self.route_widgets
                    .get_or_insert(cx, route_id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
            }

            // Trigger route change callbacks
            if let Some(new_route) = self.router.current_route() {
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn back(&mut self, cx: &mut Cx) -> bool {
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route() {
                self.active_route = route.id;
                
                // Trigger route change callbacks
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }

                if let Some(ptr) = self.route_templates.get(&route.id) {
                    self.route_widgets
                        .get_or_insert(cx, route.id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                }

                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.router.can_go_back()
    }

    pub fn current_route_id(&self) -> Option<LiveId> {
        self.router.current_route_id()
    }

    /// Navigate by path string
    pub fn navigate_by_path(&mut self, cx: &mut Cx, path: &str) -> bool {
        // First try to resolve in this router
        if let Some(route) = self.route_registry.resolve_path(path) {
            if self.route_templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                if let Some(ptr) = self.route_templates.get(&route.id) {
                    self.route_widgets
                        .get_or_insert(cx, route.id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                }

                // Trigger route change callbacks
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }

                self.redraw(cx);
                return true;
            }
        }

        // Try nested routers - check if any child router's parent route matches the beginning of the path
        let route_ids_to_check: Vec<LiveId> = self.child_routers.keys().cloned().collect();
        
        for route_id in route_ids_to_check {
            // Check if path starts with this route's pattern
            if let Some(pattern) = self.route_patterns.get(&route_id) {
                // Check if path matches the pattern or starts with it
                if let Some(route) = self.route_registry.resolve_path(path) {
                    if route.id == route_id {
                        // Path matches parent route, activate it
                        self.router.navigate(route.clone());
                        self.active_route = route.id;
                        
                        if let Some(ptr) = self.route_templates.get(&route_id) {
                            self.route_widgets
                                .get_or_insert(cx, route_id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                        }
                        
                        self.redraw(cx);
                        return true;
                    }
                }
                
                // Try to match pattern and extract remaining path
                if let Some(pattern_obj) = self.route_registry.get_pattern(route_id) {
                    if let Some(params) = pattern_obj.matches(path) {
                        // Pattern matches, create route and navigate
                        let route = Route {
                            id: route_id,
                            params,
                            pattern: Some(pattern_obj.clone()),
                        };
                        self.router.navigate(route.clone());
                        self.active_route = route.id;
                        
                        if let Some(ptr) = self.route_templates.get(&route_id) {
                            self.route_widgets
                                .get_or_insert(cx, route_id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                        }
                        
                        self.redraw(cx);
                        return true;
                    }
                }
            }
        }

        log!("Router: No route found for path: {}", path);
        false
    }

    /// Register a child router
    pub fn register_child_router(&mut self, route_id: LiveId, child: RouterWidgetRef) {
        self.child_routers.insert(route_id, child);
    }

    /// Register a route pattern
    pub fn register_route_pattern(&mut self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        self.route_registry.register_pattern(pattern, route_id)?;
        self.route_patterns.insert(route_id, pattern.to_string());
        Ok(())
    }

    /// Apply a route widget while silencing the `route_pattern` DSL metadata.
    ///
    /// `route_pattern` is a router-level metadata field, not a property of the route page widgets.
    /// The Live apply system forwards all instance children into the instantiated widget. Instead
    /// of attempting to surgically re-run the apply process without this field (which would require
    /// reconstructing parts of the apply engine), we mark the node as "prefixed". The default
    /// `LiveHook::apply_value_unknown` handler does not warn on prefixed unknown properties, so the
    /// page widget ignores it without logging.
    fn apply_widget_silencing_route_pattern(
        cx: &mut Cx,
        apply: &mut Apply,
        instance_index: usize,
        nodes: &[LiveNode],
        widget: &mut WidgetRef,
        route_pattern_idx: Option<usize>,
    ) {
        if let Some(pattern_idx) = route_pattern_idx {
            let mut patched_nodes = nodes.to_vec();
            patched_nodes[pattern_idx].origin =
                patched_nodes[pattern_idx].origin.with_node_has_prefix(true);
            widget.apply(cx, apply, instance_index, &patched_nodes);
        } else {
            widget.apply(cx, apply, instance_index, nodes);
        }
    }

    /// Register a route change callback
    /// The callback will be called whenever the route changes, with the old route (if any) and new route
    pub fn on_route_change<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cx, Option<Route>, Route) + Send + Sync + 'static,
    {
        self.route_change_callbacks.push(Box::new(callback));
    }

    /// Automatically detect and register child routers in route widgets
    /// This method scans route widgets for nested RouterWidget instances
    /// Note: Full auto-detection requires knowing field paths, so this is a best-effort approach
    /// Users can still manually register child routers if auto-detection doesn't work
    fn detect_child_routers(&mut self, _cx: &mut Cx) {
        use std::any::TypeId;
        
        let router_widget_type_id = TypeId::of::<RouterWidget>();
        
        // Scan each route widget for nested RouterWidget instances
        for (route_id, route_widget) in self.route_widgets.iter() {
            // Skip if already registered
            if self.child_routers.contains_key(route_id) {
                continue;
            }

            // Try to find RouterWidget instances by checking the widget's type
            // We'll recursively search through the widget tree
            let mut found_routers = WidgetSet::default();
            
            // Search all widgets in the route widget tree
            // This is a broad search - we look for any RouterWidget instances
            route_widget.find_widgets(&[], WidgetCache::Yes, &mut found_routers);

            // Check if any found widget is a RouterWidget by trying to borrow it
            for widget_ref in found_routers.iter() {
                if widget_ref.borrow::<RouterWidget>().is_some() {
                    // Found a RouterWidget!
                    // Since RouterWidgetRef is generated by derive macro,
                    // we can't easily create one from WidgetRef without knowing the field path.
                    // For now, we'll store a note that a router was found, and users
                    // can manually register using register_child_router if needed.
                    // In the future, we could enhance this by using the widget's path
                    // to construct the proper RouterWidgetRef.
                    log!("RouterWidget: Found nested RouterWidget in route {:?}, but auto-registration requires manual registration via register_child_router", route_id);
                    // TODO: Implement proper RouterWidgetRef creation from WidgetRef
                    break;
                }
            }
        }
    }

    /// Navigate to a nested route
    pub fn navigate_nested(&mut self, cx: &mut Cx, path: &[LiveId], route: Route) -> bool {
        if path.is_empty() {
            // Navigate in current router
            if self.route_templates.contains_key(&route.id) {
                self.router.navigate(route.clone());
                self.active_route = route.id;

                if let Some(ptr) = self.route_templates.get(&route.id) {
                    self.route_widgets
                        .get_or_insert(cx, route.id, |cx| WidgetRef::new_from_ptr(cx, Some(*ptr)));
                }

                self.redraw(cx);
                return true;
            }
            return false;
        }

        // Navigate to child router
        let first = path[0];
        let child_router_opt = self.child_routers.get(&first).cloned();
        if let Some(child_router) = child_router_opt {
            if let Some(mut child) = child_router.borrow_mut() {
                if child.navigate_nested(cx, &path[1..], route) {
                    self.redraw(cx);
                    return true;
                }
            }
        }

        false
    }
}

impl WidgetNode for RouterWidget {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn area(&self) -> Area {
        self.area
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx)
    }

    fn find_widgets(&self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        if path.is_empty() {
            return;
        }

        // Check route widgets
        for (route_id, widget) in self.route_widgets.iter() {
            if path[0] == *route_id {
                if path.len() == 1 {
                    results.push(widget.clone());
                } else {
                    widget.find_widgets(&path[1..], cached, results);
                }
                return;
            }
        }

        // Check child routers
        for (route_id, child_router) in self.child_routers.iter() {
            if path[0] == *route_id {
                if let Some(child) = child_router.borrow() {
                    child.find_widgets(&path[1..], cached, results);
                }
                return;
            }
        }

        // Fallback: search all widgets
        for widget in self.route_widgets.values() {
            widget.find_widgets(path, cached, results);
        }
    }

    fn uid_to_widget(&self, uid: WidgetUid) -> WidgetRef {
        // Check route widgets
        for widget in self.route_widgets.values() {
            let result = widget.uid_to_widget(uid);
            if !result.is_empty() {
                return result;
            }
        }

        // Check child routers
        for child_router in self.child_routers.values() {
            if let Some(child) = child_router.borrow() {
                let result = child.uid_to_widget(uid);
                if !result.is_empty() {
                    return result;
                }
            }
        }

        WidgetRef::empty()
    }
}

impl Widget for RouterWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        // Handle events for ALL route widgets, not just the active one
        // This ensures buttons on inactive pages still generate actions
        for (route_id, widget) in self.route_widgets.iter_mut() {
            let widget_uid = widget.widget_uid();
            // Only group actions for the active route so they're properly scoped
            if *route_id == self.active_route {
                cx.group_widget_actions(uid, widget_uid, |cx| {
                    widget.handle_event(cx, event, scope)
                });
            } else {
                // For inactive routes, still handle events but don't group them
                // This allows them to generate actions that can be captured
                widget.handle_event(cx, event, scope);
            }
        }

        // Handle events for child router if active route has one
        if let Some(child_router) = self.child_routers.get_mut(&self.active_route) {
            if let Some(mut child) = child_router.borrow_mut() {
                let child_uid = child.widget_uid();
                // Group actions for the child router so they're properly scoped
                cx.group_widget_actions(uid, child_uid, |cx| {
                    child.handle_event(cx, event, scope)
                });
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        if let Some(widget) = self.route_widgets.get_mut(&self.active_route) {
            widget.draw_all(cx, scope);
        }

        // Draw child routers if active route has one
        if let Some(child_router) = self.child_routers.get(&self.active_route) {
            if let Some(mut child) = child_router.borrow_mut() {
                child.draw_all(cx, scope);
            }
        }

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

impl RouterWidgetRef {
    pub fn navigate(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate(cx, route_id)
        } else {
            false
        }
    }

    pub fn back(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.back(cx)
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.can_go_back()
        } else {
            false
        }
    }

    pub fn current_route_id(&self) -> Option<LiveId> {
        if let Some(inner) = self.borrow() {
            inner.current_route_id()
        } else {
            None
        }
    }

    pub fn current_route(&self) -> Option<Route> {
        if let Some(inner) = self.borrow() {
            inner.router.current_route().cloned()
        } else {
            None
        }
    }

    /// Get a route parameter as a string
    /// Returns None if the parameter doesn't exist or the route is not active
    pub fn get_param_string(&self, param_name: &str) -> Option<String> {
        if let Some(route) = self.current_route() {
            if let Some(param_value) = route.get_param(LiveId::from_str(param_name)) {
                return param_value.as_string(|id_str| id_str.map(|s| s.to_string()));
            }
        }
        None
    }

    /// Bind a route parameter to a label widget
    /// The formatter function is called with the parameter value to generate the label text
    pub fn bind_param_to_label<F>(
        &self,
        cx: &mut Cx,
        param_name: &str,
        label_id: LiveId,
        formatter: F,
    ) -> bool
    where
        F: Fn(&str) -> String,
    {
        if let Some(param_value) = self.get_param_string(param_name) {
            // Try to find the label widget and update it
            // This is a simplified version - in practice, you'd need to pass the full widget path
            // For now, we'll just return true if the parameter exists
            // The actual label update would need to be done by the caller with the proper widget path
            let formatted_text = formatter(&param_value);
            log!("RouterWidget: Parameter {} = {}, formatted: {}", param_name, param_value, formatted_text);
            // TODO: Implement actual label binding when we have access to the widget tree
            true
        } else {
            false
        }
    }

    pub fn navigate_by_path(&self, cx: &mut Cx, path: &str) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate_by_path(cx, path)
        } else {
            false
        }
    }

    pub fn register_child_router(&self, route_id: LiveId, child: RouterWidgetRef) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.register_child_router(route_id, child);
        }
    }

    /// Navigate to a route when a button is clicked
    /// This is a convenience method that checks if the button was clicked and navigates
    pub fn navigate_on_click(
        &self,
        cx: &mut Cx,
        actions: &Actions,
        button_id: LiveId,
        target_route: LiveId,
    ) -> bool {
        // This is a simplified version - in practice, you'd need to check the button's clicked state
        // For now, we'll provide the method signature and let users implement the actual button check
        // The actual implementation would be:
        // if self.ui.button(button_id).clicked(actions) {
        //     return self.navigate(cx, target_route);
        // }
        // false
        log!("RouterWidget: navigate_on_click called for button {:?} to route {:?}", button_id, target_route);
        // TODO: Implement actual button click detection when we have access to the UI
        false
    }

    /// Register a route change callback
    pub fn on_route_change<F>(&self, callback: F)
    where
        F: Fn(&mut Cx, Option<Route>, Route) + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.on_route_change(callback);
        }
    }

    pub fn register_route_pattern(&self, pattern: &str, route_id: LiveId) -> Result<(), String> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.register_route_pattern(pattern, route_id)
        } else {
            Err("Cannot borrow router widget".to_string())
        }
    }

    pub fn navigate_nested(&self, cx: &mut Cx, path: &[LiveId], route: Route) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate_nested(cx, path, route)
        } else {
            false
        }
    }
}
