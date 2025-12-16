use crate::{route::{Route, RouteParams, RoutePattern}, router::{Router, RouteRegistry, RouterAction}};
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
    #[live]
    not_found_route: LiveId,
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
    child_router_paths: ComponentMap<LiveId, Vec<Vec<LiveId>>>,
    #[rust]
    route_change_callbacks: Vec<Box<dyn Fn(&mut Cx, Option<Route>, Route) + Send + Sync>>,
    #[rust]
    pending_actions: Vec<RouterAction>,
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
            self.route_patterns.clear();
            self.child_router_paths.clear();
            self.child_routers.clear();
            self.router.route_registry = RouteRegistry::default();
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
                        self.router.persist_state = self.persist_state;
                        self.router.reset(Route::new(initial_route));
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
                            if let Err(e) = self.router.register_route_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            }
                        } else if let LiveValue::String(pattern) = &pattern_node.value {
                            let pattern_str = pattern.as_str().to_string();
                            self.route_patterns.insert(id, pattern_str.clone());
                            // Auto-register the pattern
                            if let Err(e) = self.router.register_route_pattern(&pattern_str, id) {
                                log!("Failed to register route pattern {}: {}", pattern_str, e);
                            }
                        }
                    }

                    // Scan for nested RouterWidget instances inside this route.
                    self.child_router_paths
                        .insert(id, Self::collect_child_router_paths(index, nodes));

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
    fn resolve_nested_prefix(&self, path: &str) -> Option<(LiveId, RouteParams, RoutePattern, String)> {
        let route_ids_to_check: Vec<LiveId> = self.child_routers.keys().cloned().collect();
        let mut best: Option<(LiveId, RouteParams, RoutePattern, String, usize)> = None;

        for route_id in route_ids_to_check {
            let Some(pattern_obj) = self.router.route_registry.get_pattern(route_id) else { continue };
            let Some((params, tail)) = pattern_obj.matches_prefix_with_tail(path) else { continue };
            let priority = pattern_obj.priority();
            match &best {
                Some((_id, _p, _pat, _tail, best_prio)) if *best_prio <= priority => {}
                _ => {
                    best = Some((route_id, params, pattern_obj.clone(), tail, priority));
                }
            }
        }

        best.map(|(id, params, pattern, tail, _prio)| (id, params, pattern, tail))
    }

    fn delegate_tail_to_child(&mut self, cx: &mut Cx, parent_route_id: LiveId, tail: &str) -> bool {
        if tail.is_empty() {
            return true;
        }
        self.detect_child_routers(cx);
        let child_router = self.child_routers.get(&parent_route_id).cloned();
        if let Some(child_router) = child_router {
            if let Some(mut child) = child_router.borrow_mut() {
                return child.navigate_by_path(cx, tail);
            }
        }
        false
    }

    fn queue_route_actions(
        &mut self,
        primary_action: Option<RouterAction>,
        old_route_id: Option<LiveId>,
        new_route: &Route,
    ) {
        if let Some(primary_action) = primary_action {
            self.pending_actions.push(primary_action);
        }
        self.pending_actions.push(RouterAction::RouteChanged {
            from: old_route_id,
            to: new_route.id,
        });
    }

    fn flush_router_actions(&mut self, cx: &mut Cx, scope: &mut Scope) {
        if self.pending_actions.is_empty() {
            return;
        }
        let uid = self.widget_uid();
        for action in self.pending_actions.drain(..) {
            cx.widget_action(uid, &scope.path, action);
        }
    }

    fn new_route_widget_from_ptr(cx: &mut Cx, ptr: LivePtr) -> WidgetRef {
        let mut widget = WidgetRef::empty();
        cx.get_nodes_from_live_ptr(ptr, |cx, file_id, index, nodes| {
            let route_pattern_idx =
                nodes.child_by_name(index, LiveProp(live_id!(route_pattern), LivePropType::Field));
            let mut apply = ApplyFrom::NewFromDoc { file_id }.into();
            Self::apply_widget_silencing_route_pattern(
                cx,
                &mut apply,
                index,
                nodes,
                &mut widget,
                route_pattern_idx,
            );
            nodes.skip_node(index)
        });
        widget
    }

    fn ensure_route_widget(&mut self, cx: &mut Cx, route_id: LiveId) {
        if self.route_widgets.contains_key(&route_id) {
            return;
        }
        let Some(ptr) = self.route_templates.get(&route_id).copied() else {
            return;
        };
        self.route_widgets
            .get_or_insert(cx, route_id, |cx| Self::new_route_widget_from_ptr(cx, ptr));
    }

    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if self.route_templates.contains_key(&route_id) {
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);

            if let Some(new_route) = self.router.current_route().cloned() {
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Navigate(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
            }

            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn replace(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if self.route_templates.contains_key(&route_id) {
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);

            if let Some(new_route) = self.router.current_route().cloned() {
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Replace(new_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &new_route,
                );
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
            if let Some(route) = self.router.current_route().cloned() {
                self.active_route = route.id;
                
                // Trigger route change callbacks
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }

                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn forward(&mut self, cx: &mut Cx) -> bool {
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.active_route = route.id;
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
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

    pub fn can_go_forward(&self) -> bool {
        self.router.can_go_forward()
    }

    pub fn depth(&self) -> usize {
        self.router.depth()
    }

    pub fn current_route_id(&self) -> Option<LiveId> {
        self.router.current_route_id()
    }

    pub fn clear_history(&mut self, cx: &mut Cx) {
        self.router.clear_history();
        self.redraw(cx);
    }

    pub fn reset(&mut self, cx: &mut Cx, route: Route) -> bool {
        if !self.route_templates.contains_key(&route.id) {
            log!("Router: Route template not found for {:?}", route.id);
            return false;
        }
        let old_route = self.router.current_route().cloned();
        self.router.reset(route.clone());
        self.active_route = route.id;
        self.ensure_route_widget(cx, route.id);

        if let Some(new_route) = self.router.current_route().cloned() {
            for callback in &self.route_change_callbacks {
                callback(cx, old_route.clone(), new_route.clone());
            }
            self.queue_route_actions(
                Some(RouterAction::Reset(new_route.clone())),
                old_route.as_ref().map(|r| r.id),
                &new_route,
            );
        }

        self.redraw(cx);
        true
    }

    pub fn push(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.navigate(cx, route_id)
    }

    pub fn pop(&mut self, cx: &mut Cx) -> bool {
        let old_route = self.router.current_route().cloned();
        if self.router.pop() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to(route_id) {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to_root(&mut self, cx: &mut Cx) -> bool {
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to_root() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn set_stack(&mut self, cx: &mut Cx, stack: Vec<Route>) -> bool {
        let filtered: Vec<Route> = stack
            .into_iter()
            .filter(|r| self.route_templates.contains_key(&r.id))
            .collect();
        if filtered.is_empty() {
            return false;
        }
        let old_route = self.router.current_route().cloned();
        self.router.set_stack(filtered);
        let Some(new_route) = self.router.current_route().cloned() else { return false };
        self.active_route = new_route.id;
        self.ensure_route_widget(cx, new_route.id);
        for callback in &self.route_change_callbacks {
            callback(cx, old_route.clone(), new_route.clone());
        }
        self.queue_route_actions(
            Some(RouterAction::Reset(new_route.clone())),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );
        self.redraw(cx);
        true
    }

    /// Navigate by path string
    pub fn navigate_by_path(&mut self, cx: &mut Cx, path: &str) -> bool {
        // 1) Full match in this router.
        if let Some(route) = self.router.route_registry.resolve_path(path) {
            if self.route_templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                self.ensure_route_widget(cx, route.id);

                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Navigate(route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                // If this route owns a child router, delegate the tail to it.
                if self.child_routers.contains_key(&route.id) {
                    if let Some(pattern) = &route.pattern {
                        if let Some((_params, tail)) = pattern.matches_prefix_with_tail(path) {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }

                self.redraw(cx);
                return true;
            }
        }

        // 2) Prefix match for nested routing: activate a parent route and delegate the tail.
        if let Some((route_id, params, pattern, tail)) = self.resolve_nested_prefix(path) {
            if self.route_templates.contains_key(&route_id) {
                let old_route = self.router.current_route().cloned();
                let parent_route = Route {
                    id: route_id,
                    params,
                    pattern: Some(pattern),
                };
                self.router.navigate(parent_route.clone());
                self.active_route = route_id;
                self.ensure_route_widget(cx, route_id);

                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), parent_route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Navigate(parent_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &parent_route,
                );

                let _ = self.delegate_tail_to_child(cx, route_id, &tail);
                self.redraw(cx);
                return true;
            }
        }

        // 3) Not-found fallback.
        if self.not_found_route.0 != 0 && self.route_templates.contains_key(&self.not_found_route) {
            // Push not-found so the user can navigate back to the previous page.
            // If we're already on not-found, don't keep growing history.
            if self.current_route_id() == Some(self.not_found_route) {
                return false;
            }
            return self.navigate(cx, self.not_found_route);
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
        self.router.register_route_pattern(pattern, route_id)?;
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

    /// Automatically detect and register child routers in route widgets.
    ///
    /// We scan the Live DSL for nested `RouterWidget` instances (and their widget-id paths) in
    /// `apply_value_instance`, then resolve those paths against the instantiated route widgets here.
    fn detect_child_routers(&mut self, _cx: &mut Cx) {
        for (route_id, route_widget) in self.route_widgets.iter() {
            if self.child_routers.contains_key(route_id) {
                continue;
            }
            let Some(paths) = self.child_router_paths.get(route_id) else { continue };
            for path in paths {
                let child_widget = route_widget.widget(path);
                if child_widget.borrow::<RouterWidget>().is_some() {
                    self.child_routers.insert(*route_id, child_widget.as_router_widget());
                    break;
                }
            }
        }
    }

    fn collect_child_router_paths(root_index: usize, nodes: &[LiveNode]) -> Vec<Vec<LiveId>> {
        let router_live_type = LiveType::of::<RouterWidget>();
        let mut out = Vec::new();
        let mut path = Vec::<LiveId>::new();

        let end = nodes.skip_node(root_index);
        let mut i = root_index + 1;
        while i < end {
            i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, &mut path, &mut out);
        }
        out
    }

    fn collect_child_router_paths_recur(
        index: usize,
        nodes: &[LiveNode],
        router_live_type: LiveType,
        path: &mut Vec<LiveId>,
        out: &mut Vec<Vec<LiveId>>,
    ) -> usize {
        let node = &nodes[index];

        if node.origin.has_prop_type(LivePropType::Instance) {
            if let LiveValue::Class { live_type, .. } = &node.value {
                path.push(node.id);
                if *live_type == router_live_type {
                    out.push(path.clone());
                }

                let end = nodes.skip_node(index);
                let mut i = index + 1;
                while i < end {
                    i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, path, out);
                }
                path.pop();
                return end;
            }
        }

        if node.value.is_open() {
            let end = nodes.skip_node(index);
            let mut i = index + 1;
            while i < end {
                i = Self::collect_child_router_paths_recur(i, nodes, router_live_type, path, out);
            }
            return end;
        }

        index + 1
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
        self.flush_router_actions(cx, scope);
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
    fn with_active_route_widget<R>(&self, f: impl FnOnce(&WidgetRef) -> R) -> Option<R> {
        let inner = self.borrow()?;
        let active_route = inner.active_route;
        let route_widget = inner.route_widgets.get(&active_route)?;
        Some(f(route_widget))
    }

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

    pub fn replace(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.replace(cx, route_id)
        } else {
            false
        }
    }

    pub fn forward(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.forward(cx)
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

    pub fn can_go_forward(&self) -> bool {
        if let Some(inner) = self.borrow() {
            inner.can_go_forward()
        } else {
            false
        }
    }

    pub fn depth(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.depth()
        } else {
            0
        }
    }

    pub fn clear_history(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.clear_history(cx);
        }
    }

    pub fn reset(&self, cx: &mut Cx, route: Route) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.reset(cx, route)
        } else {
            false
        }
    }

    pub fn push(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.push(cx, route_id)
        } else {
            false
        }
    }

    pub fn pop(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pop(cx)
        } else {
            false
        }
    }

    pub fn pop_to(&self, cx: &mut Cx, route_id: LiveId) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pop_to(cx, route_id)
        } else {
            false
        }
    }

    pub fn pop_to_root(&self, cx: &mut Cx) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.pop_to_root(cx)
        } else {
            false
        }
    }

    pub fn set_stack(&self, cx: &mut Cx, stack: Vec<Route>) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_stack(cx, stack)
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
            let formatted_text = formatter(&param_value);
            self.with_active_route_widget(|route_widget| {
                let label = route_widget.widget(&[label_id]);
                if label.is_empty() {
                    return false;
                }
                label.set_text(cx, &formatted_text);
                true
            })
            .unwrap_or(false)
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
        if self
            .with_active_route_widget(|route_widget| route_widget.button(&[button_id]).clicked(actions))
            .unwrap_or(false)
        {
            return self.navigate(cx, target_route);
        }
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
