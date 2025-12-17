use crate::{
    guards::{
        RouterAsyncDecision, RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveDecision,
        RouterBeforeLeaveSync, RouterGuardDecision, RouterNavContext,
        RouterSyncGuard,
    },
    hero::{HeroGlobals, HeroPhase},
    route::{Route, RouteQuery},
    router::{RouteRegistry, Router, RouterAction},
    state::RouterState,
    url::RouterUrl,
};
use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::*;

pub use crate::hero::Hero;

mod inspector;
mod transitions;
mod url_sync;
mod nested;
mod guard_flow;

pub use transitions::{RouterTransitionPreset, RouterTransitionSpec};
use transitions::{RouterActionKind, RouterTransitionDirection, RouterTransitionState};
use guard_flow::PendingNavigation;

live_design! {
    link widgets;
    use link::theme::*;
    use makepad_draw::shader::std::*;

    DrawInspectorRect = {{DrawInspectorRect}} {
        fn pixel(self) -> vec4 {
            return self.color;
        }
    }

    pub RouterWidgetBase = {{RouterWidget}} {
        flow: Overlay
        clip_x: true
        clip_y: true

        // Phase 3: transitions/animations (default off).
        push_transition: none
        pop_transition: none
        replace_transition: none
        transition_duration: 0.25
        hero_transition: false
        debug_inspector: false
        inspector_bg: {draw_depth: 10.0, color: #x00000012}
        inspector_text: {
            text_style: <THEME_FONT_REGULAR> {font_size: 9}
            color: #xFFFFFFFF
            draw_depth: 11.0
        }

        // Phase 4: URL + deep linking (web only).
        url_sync: true
        use_initial_url: false
    }
    pub RouterWidget = <RouterWidgetBase> {
        width: Fill, height: Fill
    }

    pub Hero = {{Hero}} {
        width: Fit, height: Fit
    }
}

#[derive(Clone, Debug)]
enum RouterNavRequest {
    Navigate {
        route_id: LiveId,
    },
    NavigateWithTransition {
        route_id: LiveId,
        transition: RouterTransitionSpec,
    },
    Replace {
        route_id: LiveId,
    },
    ReplaceWithTransition {
        route_id: LiveId,
        transition: RouterTransitionSpec,
    },
    NavigateByPath {
        path: String,
    },
    ReplaceByPath {
        path: String,
        clear_extras: bool,
    },
    NavigateByUrl {
        url: String,
    },
    ReplaceByUrl {
        url: String,
    },
    Back {
        transition: Option<RouterTransitionSpec>,
    },
    Forward {
        transition: Option<RouterTransitionSpec>,
    },
    Reset {
        route: Route,
    },
    SetStack {
        stack: Vec<Route>,
    },
    Pop,
    PopTo {
        route_id: LiveId,
    },
    PopToRoot,
    #[cfg(target_arch = "wasm32")]
    BrowserUrlChanged {
        url: String,
        state_index: i32,
    },
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
    /// Sync route changes into the browser URL/history on web (wasm32).
    #[live(true)]
    url_sync: bool,
    /// When `url_sync` is enabled, use the initial browser URL on startup (web only).
    #[live(false)]
    use_initial_url: bool,
    #[live(false)]
    persist_state: bool,
    /// Default transition used for push/navigate.
    #[live]
    push_transition: LiveId,
    /// Default transition used for back/pop.
    #[live]
    pop_transition: LiveId,
    /// Default transition used for replace/reset/set_stack.
    #[live]
    replace_transition: LiveId,
    /// Default transition duration (seconds).
    #[live(0.25)]
    transition_duration: f64,
    /// Enables shared-element ("hero") transitions between routes.
    #[live(false)]
    hero_transition: bool,
    /// Shows a small debug overlay with current route/stack/params (dev tool).
    #[live(false)]
    debug_inspector: bool,
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
    route_transition_overrides: ComponentMap<LiveId, LiveId>,
    #[rust]
    route_transition_duration_overrides: ComponentMap<LiveId, f64>,
    #[rust]
    child_router_paths: ComponentMap<LiveId, Vec<Vec<LiveId>>>,
    #[rust]
    route_change_callbacks: Vec<Box<dyn Fn(&mut Cx, Option<Route>, Route) + Send + Sync>>,
    #[rust]
    route_guards: Vec<RouterSyncGuard>,
    #[rust]
    route_guards_async: Vec<RouterAsyncGuard>,
    #[rust]
    before_leave_hooks: Vec<RouterBeforeLeaveSync>,
    #[rust]
    before_leave_hooks_async: Vec<RouterBeforeLeaveAsync>,
    #[rust]
    pending_navigation: Option<PendingNavigation>,
    #[rust]
    guard_bypass: bool,
    #[rust]
    pending_actions: Vec<RouterAction>,
    #[rust]
    url_path_override: Option<String>,
    #[rust]
    web_history_index: i32,
    #[rust]
    web_history_initialized: bool,
    #[rust]
    suppress_browser_update: bool,
    #[rust]
    ignore_next_browser_url_change: bool,
    #[rust]
    web_last_synced_url: Option<String>,
    #[rust]
    web_last_depth: usize,
    #[rust]
    web_last_child_depth: Option<usize>,
    #[rust]
    web_last_child_parent_route: LiveId,
    #[rust(DrawList2d::new(cx))]
    from_draw_list: DrawList2d,
    #[rust(DrawList2d::new(cx))]
    to_draw_list: DrawList2d,
    #[rust(DrawList2d::new(cx))]
    hero_capture_draw_list: DrawList2d,
    #[rust(DrawList2d::new(cx))]
    hero_from_draw_list: DrawList2d,
    #[rust(DrawList2d::new(cx))]
    hero_to_draw_list: DrawList2d,
    #[rust(DrawList2d::new(cx))]
    inspector_draw_list: DrawList2d,
    #[live]
    inspector_bg: DrawInspectorRect,
    #[live]
    inspector_text: DrawText,
    #[rust]
    transition: Option<RouterTransitionState>,
    #[rust]
    transition_next_frame: NextFrame,
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
            self.route_transition_overrides.clear();
            self.route_transition_duration_overrides.clear();
            self.child_router_paths.clear();
            self.child_routers.clear();
            self.router.route_registry = RouteRegistry::default();
            self.transition = None;
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
                                    let route_transition_idx = nodes.child_by_name(
                                        index,
                                        LiveProp(live_id!(route_transition), LivePropType::Field),
                                    );
                                    let route_transition_duration_idx = nodes.child_by_name(
                                        index,
                                        LiveProp(
                                            live_id!(route_transition_duration),
                                            LivePropType::Field,
                                        ),
                                    );
                                    apply.override_from(
                                        ApplyFrom::NewFromDoc { file_id },
                                        |apply| {
                                            Self::apply_widget_silencing_route_metadata(
                                                cx,
                                                apply,
                                                index,
                                                nodes,
                                                &mut widget,
                                                &[
                                                    route_pattern_idx,
                                                    route_transition_idx,
                                                    route_transition_duration_idx,
                                                ],
                                            );
                                        },
                                    );
                                    nodes.skip_node(index)
                                });
                                widget
                            });
                        }
                    }
                }

                // Auto-detect child routers in route widgets
                self.detect_child_routers(cx);
                self.apply_initial_url_if_needed(cx);
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
                    if let Some(pattern_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_pattern), LivePropType::Field),
                    ) {
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

                    // Optional per-route transition override (DSL metadata).
                    if let Some(transition_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition), LivePropType::Field),
                    ) {
                        let transition_node = &nodes[transition_node_idx];
                        let transition_id = match &transition_node.value {
                            LiveValue::Id(id) => *id,
                            LiveValue::Str(s) => LiveId::from_str(s),
                            LiveValue::String(s) => LiveId::from_str(s.as_str()),
                            _ => LiveId(0),
                        };
                        if transition_id.0 != 0 {
                            self.route_transition_overrides.insert(id, transition_id);
                        }
                    }

                    if let Some(duration_node_idx) = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition_duration), LivePropType::Field),
                    ) {
                        let duration_node = &nodes[duration_node_idx];
                        let duration = match &duration_node.value {
                            LiveValue::Float64(v) => Some(*v),
                            LiveValue::Float32(v) => Some(*v as f64),
                            LiveValue::Int64(v) => Some(*v as f64),
                            _ => None,
                        };
                        if let Some(duration) = duration {
                            self.route_transition_duration_overrides
                                .insert(id, duration);
                        }
                    }

                    // Scan for nested RouterWidget instances inside this route.
                    self.child_router_paths
                        .insert(id, Self::collect_child_router_paths(index, nodes));

                    // Create/update the route widget instance. We silence `route_pattern` by marking
                    // it as a prefixed property before applying, so it is ignored by the default
                    // `apply_value_unknown` handler (no noisy "no matching field" warning).
                    let route_pattern_idx = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_pattern), LivePropType::Field),
                    );
                    let route_transition_idx = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition), LivePropType::Field),
                    );
                    let route_transition_duration_idx = nodes.child_by_name(
                        index,
                        LiveProp(live_id!(route_transition_duration), LivePropType::Field),
                    );

                    let widget = self
                        .route_widgets
                        .get_or_insert(cx, id, |_cx| WidgetRef::empty());

                    Self::apply_widget_silencing_route_metadata(
                        cx,
                        apply,
                        index,
                        nodes,
                        widget,
                        &[
                            route_pattern_idx,
                            route_transition_idx,
                            route_transition_duration_idx,
                        ],
                    );
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
            let route_pattern_idx = nodes.child_by_name(
                index,
                LiveProp(live_id!(route_pattern), LivePropType::Field),
            );
            let route_transition_idx = nodes.child_by_name(
                index,
                LiveProp(live_id!(route_transition), LivePropType::Field),
            );
            let route_transition_duration_idx = nodes.child_by_name(
                index,
                LiveProp(live_id!(route_transition_duration), LivePropType::Field),
            );
            let mut apply = ApplyFrom::NewFromDoc { file_id }.into();
            Self::apply_widget_silencing_route_metadata(
                cx,
                &mut apply,
                index,
                nodes,
                &mut widget,
                &[
                    route_pattern_idx,
                    route_transition_idx,
                    route_transition_duration_idx,
                ],
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

    pub fn navigate_by_url(&mut self, cx: &mut Cx, url: &str) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateByUrl {
                    url: url.to_string(),
                },
            );
        }
        let parsed = RouterUrl::parse(url);
        self.ensure_web_history_initialized(cx);

        let ok = self.navigate_by_path_internal(cx, &parsed.to_string(), false);

        if ok {
            self.web_push_current_url(cx);
        }
        ok
    }

    pub fn navigate(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Navigate { route_id });
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Push,
                RouterTransitionDirection::Forward,
                None,
            );

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

            self.web_push_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn navigate_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.navigate_to(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Push,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

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

            self.web_push_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn replace(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Replace { route_id });
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                None,
            );

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

            self.web_replace_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn replace_with_transition(
        &mut self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::ReplaceWithTransition {
                    route_id,
                    transition,
                },
            );
        }
        if self.route_templates.contains_key(&route_id) {
            self.clear_url_extras();
            let old_route = self.router.current_route().cloned();
            self.router.replace_with(route_id);
            self.active_route = route_id;

            self.ensure_route_widget(cx, route_id);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                route_id,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                Some(transition),
            );

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

            self.web_replace_current_url(cx);
            self.redraw(cx);
            true
        } else {
            log!("Router: Route template not found for {:?}", route_id);
            false
        }
    }

    pub fn back(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Back { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );

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

                self.web_go(cx, -1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn back_with_transition(&mut self, cx: &mut Cx, transition: RouterTransitionSpec) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Back {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.back() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    Some(transition),
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Back),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, -1);
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
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Forward { transition: None });
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, 1);
                self.redraw(cx);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn forward_with_transition(
        &mut self,
        cx: &mut Cx,
        transition: RouterTransitionSpec,
    ) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::Forward {
                    transition: Some(transition),
                },
            );
        }
        let old_route = self.router.current_route().cloned();
        if self.router.forward() {
            if let Some(route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = route.id;
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    Some(transition),
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.ensure_route_widget(cx, route.id);
                self.queue_route_actions(
                    Some(RouterAction::Forward),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );
                self.web_go(cx, 1);
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

    pub fn get_state(&self) -> RouterState {
        RouterState {
            history: self.router.history.clone(),
            url_path_override: self.url_path_override.clone(),
        }
    }

    pub fn set_state(&mut self, cx: &mut Cx, state: RouterState) -> bool {
        let old_route = self.router.current_route().cloned();
        let (stack, current_index) = state.history.into_parts();

        let mut filtered = Vec::<Route>::new();
        let mut new_current = 0usize;
        for (idx, route) in stack.into_iter().enumerate() {
            if !self.route_templates.contains_key(&route.id) {
                continue;
            }
            if idx <= current_index {
                new_current = filtered.len();
            }
            filtered.push(route);
        }
        if filtered.is_empty() {
            return false;
        }

        self.clear_url_extras();
        self.url_path_override = state.url_path_override;
        self.router.history = crate::navigation::NavigationHistory::from_parts(filtered, new_current);
        let Some(new_route) = self.router.current_route().cloned() else {
            return false;
        };
        self.active_route = new_route.id;
        self.transition = None;
        self.ensure_route_widget(cx, new_route.id);

        for callback in &self.route_change_callbacks {
            callback(cx, old_route.clone(), new_route.clone());
        }
        self.queue_route_actions(
            Some(RouterAction::Reset(new_route.clone())),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );

        self.web_replace_current_url(cx);
        self.redraw(cx);
        true
    }

    pub fn clear_history(&mut self, cx: &mut Cx) {
        self.router.clear_history();
        self.web_replace_current_url(cx);
        self.redraw(cx);
    }

    pub fn reset(&mut self, cx: &mut Cx, route: Route) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Reset { route });
        }
        if !self.route_templates.contains_key(&route.id) {
            log!("Router: Route template not found for {:?}", route.id);
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.reset(route.clone());
        self.active_route = route.id;
        self.ensure_route_widget(cx, route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            route.id,
            RouterActionKind::Replace,
            RouterTransitionDirection::Forward,
            None,
        );

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

        self.web_replace_current_url(cx);
        self.redraw(cx);
        true
    }

    pub fn push(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        self.navigate(cx, route_id)
    }

    pub fn pop(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::Pop);
        }
        let old_route = self.router.current_route().cloned();
        if self.router.pop() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                self.web_go(cx, -1);
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to(&mut self, cx: &mut Cx, route_id: LiveId) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopTo { route_id });
        }
        let before_depth = self.router.depth() as i32;
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to(route_id) {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                let after_depth = self.router.depth() as i32;
                let delta = after_depth - before_depth;
                if delta != 0 {
                    self.web_go(cx, delta);
                }
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn pop_to_root(&mut self, cx: &mut Cx) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::PopToRoot);
        }
        let before_depth = self.router.depth() as i32;
        let old_route = self.router.current_route().cloned();
        if self.router.pop_to_root() {
            if let Some(new_route) = self.router.current_route().cloned() {
                self.clear_url_extras();
                self.active_route = new_route.id;
                self.ensure_route_widget(cx, new_route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    new_route.id,
                    RouterActionKind::Pop,
                    RouterTransitionDirection::Backward,
                    None,
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), new_route.clone());
                }
                self.queue_route_actions(None, old_route.as_ref().map(|r| r.id), &new_route);
                let after_depth = self.router.depth() as i32;
                let delta = after_depth - before_depth;
                if delta != 0 {
                    self.web_go(cx, delta);
                }
                self.redraw(cx);
                return true;
            }
        }
        false
    }

    pub fn set_stack(&mut self, cx: &mut Cx, stack: Vec<Route>) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(cx, RouterNavRequest::SetStack { stack });
        }
        let filtered: Vec<Route> = stack
            .into_iter()
            .filter(|r| self.route_templates.contains_key(&r.id))
            .collect();
        if filtered.is_empty() {
            return false;
        }
        self.clear_url_extras();
        let old_route = self.router.current_route().cloned();
        self.router.set_stack(filtered);
        let Some(new_route) = self.router.current_route().cloned() else {
            return false;
        };
        self.active_route = new_route.id;
        self.ensure_route_widget(cx, new_route.id);
        self.start_transition(
            cx,
            old_route.as_ref().map(|r| r.id),
            new_route.id,
            RouterActionKind::Replace,
            RouterTransitionDirection::Forward,
            None,
        );
        for callback in &self.route_change_callbacks {
            callback(cx, old_route.clone(), new_route.clone());
        }
        self.queue_route_actions(
            Some(RouterAction::Reset(new_route.clone())),
            old_route.as_ref().map(|r| r.id),
            &new_route,
        );
        self.web_replace_current_url(cx);
        self.redraw(cx);
        true
    }

    /// Navigate by path string
    pub fn navigate_by_path(&mut self, cx: &mut Cx, path: &str) -> bool {
        if !self.guard_bypass {
            return self.request_navigation(
                cx,
                RouterNavRequest::NavigateByPath {
                    path: path.to_string(),
                },
            );
        }
        let ok = self.navigate_by_path_internal(cx, path, true);
        if ok {
            self.web_push_current_url(cx);
        }
        ok
    }

    fn navigate_by_path_internal(&mut self, cx: &mut Cx, path: &str, clear_extras: bool) -> bool {
        let parsed = RouterUrl::parse(path);
        let query = RouteQuery::from_query_string(&parsed.query);
        let hash = parsed.hash.clone();
        let path = parsed.path;

        if clear_extras {
            self.clear_url_extras();
        } else {
            self.url_path_override = None;
        }

        // 1) Full match in this router.
        if let Some(mut route) = self.router.route_registry.resolve_path(&path) {
            if self.route_templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                route.query = query.clone();
                route.hash = hash.clone();
                self.router.navigate(route.clone());
                self.active_route = route.id;

                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );

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
                        if let Some((_params, tail)) = pattern.matches_prefix_with_tail(&path) {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }

                self.redraw(cx);
                return true;
            }
        }

        // 2) Prefix match for nested routing: activate a parent route and delegate the tail.
        if let Some((route_id, params, pattern, tail)) = self.resolve_nested_prefix(&path) {
            if self.route_templates.contains_key(&route_id) {
                let old_route = self.router.current_route().cloned();
                let parent_route = Route {
                    id: route_id,
                    params,
                    query: query.clone(),
                    hash: hash.clone(),
                    pattern: Some(pattern),
                };
                self.router.navigate(parent_route.clone());
                self.active_route = route_id;
                self.ensure_route_widget(cx, route_id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route_id,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );

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
            // Preserve the attempted path in the address bar.
            if self.current_route_id() != Some(self.not_found_route) {
                self.url_path_override = Some(path);
                let old_route = self.router.current_route().cloned();
                let mut nf = Route::new(self.not_found_route);
                nf.query = query;
                nf.hash = hash;
                self.router.navigate(nf);
                self.active_route = self.not_found_route;
                self.ensure_route_widget(cx, self.not_found_route);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    self.not_found_route,
                    RouterActionKind::Push,
                    RouterTransitionDirection::Forward,
                    None,
                );
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
                return true;
            }
            return false;
        }

        log!("Router: No route found for path: {}", path);
        false
    }

    fn replace_by_path_internal(&mut self, cx: &mut Cx, path: &str, clear_extras: bool) -> bool {
        let parsed = RouterUrl::parse(path);
        let query = RouteQuery::from_query_string(&parsed.query);
        let hash = parsed.hash.clone();
        let path = parsed.path;

        if clear_extras {
            self.clear_url_extras();
        } else {
            self.url_path_override = None;
        }

        if let Some(mut route) = self.router.route_registry.resolve_path(&path) {
            if self.route_templates.contains_key(&route.id) {
                let old_route = self.router.current_route().cloned();
                route.query = query.clone();
                route.hash = hash.clone();
                self.router.replace(route.clone());
                self.active_route = route.id;
                self.ensure_route_widget(cx, route.id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route.id,
                    RouterActionKind::Replace,
                    RouterTransitionDirection::Forward,
                    None,
                );
                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Replace(route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &route,
                );

                // If this route owns a child router, delegate the tail to it.
                if self.child_routers.contains_key(&route.id) {
                    if let Some(pattern) = &route.pattern {
                        if let Some((_params, tail)) = pattern.matches_prefix_with_tail(&path) {
                            let _ = self.delegate_tail_to_child(cx, route.id, &tail);
                        }
                    }
                }

                self.redraw(cx);
                return true;
            }
        }

        if let Some((route_id, params, pattern, tail)) = self.resolve_nested_prefix(&path) {
            if self.route_templates.contains_key(&route_id) {
                let old_route = self.router.current_route().cloned();
                let parent_route = Route {
                    id: route_id,
                    params,
                    query: query.clone(),
                    hash: hash.clone(),
                    pattern: Some(pattern),
                };
                self.router.replace(parent_route.clone());
                self.active_route = route_id;
                self.ensure_route_widget(cx, route_id);
                self.start_transition(
                    cx,
                    old_route.as_ref().map(|r| r.id),
                    route_id,
                    RouterActionKind::Replace,
                    RouterTransitionDirection::Forward,
                    None,
                );

                for callback in &self.route_change_callbacks {
                    callback(cx, old_route.clone(), parent_route.clone());
                }
                self.queue_route_actions(
                    Some(RouterAction::Replace(parent_route.clone())),
                    old_route.as_ref().map(|r| r.id),
                    &parent_route,
                );

                let _ = self.delegate_tail_to_child(cx, route_id, &tail);
                self.redraw(cx);
                return true;
            }
        }

        if self.not_found_route.0 != 0 && self.route_templates.contains_key(&self.not_found_route) {
            self.url_path_override = Some(path);
            let old_route = self.router.current_route().cloned();
            let mut nf = Route::new(self.not_found_route);
            nf.query = query;
            nf.hash = hash;
            self.router.replace(nf);
            self.active_route = self.not_found_route;
            self.ensure_route_widget(cx, self.not_found_route);
            self.start_transition(
                cx,
                old_route.as_ref().map(|r| r.id),
                self.not_found_route,
                RouterActionKind::Replace,
                RouterTransitionDirection::Forward,
                None,
            );
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
            return true;
        }

        false
    }

    /// Register a child router
    pub fn register_child_router(&mut self, route_id: LiveId, child: RouterWidgetRef) {
        if let Some(mut inner) = child.borrow_mut() {
            inner.url_sync = false;
            inner.use_initial_url = false;
            inner.web_history_initialized = false;
        }
        self.child_routers.insert(route_id, child);
    }

    /// Register a route pattern
    pub fn register_route_pattern(
        &mut self,
        pattern: &str,
        route_id: LiveId,
    ) -> Result<(), String> {
        self.router.register_route_pattern(pattern, route_id)?;
        self.route_patterns.insert(route_id, pattern.to_string());
        Ok(())
    }

    /// Apply a route widget while silencing router-only DSL metadata.
    ///
    /// `route_pattern` / `route_transition` / `route_transition_duration` are router-level metadata
    /// fields, not properties of the route page widgets.
    /// The Live apply system forwards all instance children into the instantiated widget. Instead
    /// of attempting to surgically re-run the apply process without this field (which would require
    /// reconstructing parts of the apply engine), we mark the node as "prefixed". The default
    /// `LiveHook::apply_value_unknown` handler does not warn on prefixed unknown properties, so the
    /// page widget ignores it without logging.
    fn apply_widget_silencing_route_metadata(
        cx: &mut Cx,
        apply: &mut Apply,
        instance_index: usize,
        nodes: &[LiveNode],
        widget: &mut WidgetRef,
        silence_node_indices: &[Option<usize>],
    ) {
        if silence_node_indices.iter().all(|i| i.is_none()) {
            widget.apply(cx, apply, instance_index, nodes);
            return;
        }

        let mut patched_nodes = nodes.to_vec();
        for idx in silence_node_indices.iter().flatten().copied() {
            patched_nodes[idx].origin = patched_nodes[idx].origin.with_node_has_prefix(true);
        }
        widget.apply(cx, apply, instance_index, &patched_nodes);
    }

    /// Register a route change callback
    /// The callback will be called whenever the route changes, with the old route (if any) and new route
    pub fn on_route_change<F>(&mut self, callback: F)
    where
        F: Fn(&mut Cx, Option<Route>, Route) + Send + Sync + 'static,
    {
        self.route_change_callbacks.push(Box::new(callback));
    }

    pub fn add_route_guard<F>(&mut self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync + 'static,
    {
        self.route_guards.push(Box::new(guard));
    }

    pub fn add_route_guard_async<F>(&mut self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision>
            + Send
            + Sync
            + 'static,
    {
        self.route_guards_async.push(Box::new(guard));
    }

    pub fn add_before_leave_hook<F>(&mut self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync + 'static,
    {
        self.before_leave_hooks.push(Box::new(hook));
    }

    pub fn add_before_leave_hook_async<F>(&mut self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
            + Send
            + Sync
            + 'static,
    {
        self.before_leave_hooks_async.push(Box::new(hook));
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
        self.from_draw_list.redraw(cx);
        self.to_draw_list.redraw(cx);
        self.inspector_draw_list.redraw(cx);
        self.area.redraw(cx);
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
        #[cfg(target_arch = "wasm32")]
        if let Event::ToWasmMsg(msg) = event {
            if msg.id == live_id!(ToWasmBrowserUrlChanged) {
                let mut r = msg.as_ref();
                let url = r.read_string();
                let state_index = r.read_f64() as i32;
                self.handle_browser_url_changed(cx, &url, state_index);
            }
        }

        if let Some(ne) = self.transition_next_frame.is_event(event) {
            self.update_transition(cx, ne.time);
        }
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

        // Nested routers have `url_sync` disabled; sync the full (composed) URL from here.
        self.poll_pending_navigation(cx);
        self.sync_web_url_if_needed(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let mut layout = self.layout;
        layout.flow = Flow::Overlay;
        layout.clip_x = true;
        layout.clip_y = true;
        cx.begin_turtle(walk, layout);

        let rect = cx.turtle().inner_rect();
        let router_uid = self.widget_uid();
        let hero_enabled = self.hero_transition && self.transition.is_some();

        if hero_enabled {
            cx.global::<HeroGlobals>().push_router(router_uid);
        }

        if let Some(state_snapshot) = self.transition.clone() {
	            if hero_enabled && !state_snapshot.hero_capture_done {
	                cx.global::<HeroGlobals>().clear_capture();
	                cx.global::<HeroGlobals>().set_hide_tags(&[]);

	                self.hero_capture_draw_list.begin_always(cx);
	                let draw_list_id = self.hero_capture_draw_list.id();
	                {
	                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
	                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
	                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
	                    dl.draw_list_uniforms.view_opacity = 0.0;
	                }

	                cx.global::<HeroGlobals>().set_phase(HeroPhase::CaptureFrom);
	                if let Some(widget) = self.route_widgets.get_mut(&state_snapshot.from_route) {
	                    let _ = widget.draw_walk(cx, scope, Walk::fill().with_abs_pos(rect.pos));
	                }

	                cx.global::<HeroGlobals>().set_phase(HeroPhase::CaptureTo);
	                if let Some(widget) = self.route_widgets.get_mut(&state_snapshot.to_route) {
	                    let _ = widget.draw_walk(cx, scope, Walk::fill().with_abs_pos(rect.pos));
	                }

	                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
	                self.hero_capture_draw_list.end(cx);

                let hero_pairs = cx.global::<HeroGlobals>().take_pairs();
                if let Some(state) = self.transition.as_mut() {
                    state.hero_pairs = hero_pairs;
                    state.hero_capture_done = true;
                }
            }

            let state = self.transition.clone().unwrap_or(state_snapshot);

            let hide_tags: Vec<LiveId> = state.hero_pairs.iter().map(|p| p.tag).collect();
            let has_hero_pairs = hero_enabled && !hide_tags.is_empty();
            let route_preset = if has_hero_pairs {
                RouterTransitionPreset::Fade
            } else {
                state.preset
            };

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_hide_tags(&hide_tags);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::VisibleFrom);
            } else {
                cx.global::<HeroGlobals>().set_hide_tags(&[]);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }

            let from_effect =
                Self::compute_effect(route_preset, state.direction, state.progress, false, rect);
	            Self::draw_route_into_draw_list(
	                cx,
	                scope,
	                &mut self.from_draw_list,
	                &mut self.route_widgets,
	                state.from_route,
	                from_effect,
	                true,
	            );

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::VisibleTo);
            } else {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }

            let to_effect =
                Self::compute_effect(route_preset, state.direction, state.progress, true, rect);
	            Self::draw_route_into_draw_list(
	                cx,
	                scope,
	                &mut self.to_draw_list,
	                &mut self.route_widgets,
	                state.to_route,
	                to_effect,
	                true,
	            );

            if has_hero_pairs {
                cx.global::<HeroGlobals>().set_hide_tags(&[]);
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Overlay);

                let t = Self::ease_in_out(state.progress);
                let from_opacity = (1.0 - t) as f32;
                let to_opacity = t as f32;

                let pairs = state.hero_pairs.clone();
                let lerp = |a: f64, b: f64| a + (b - a) * t;
	                let lerp_rect = |a: Rect, b: Rect| Rect {
	                    pos: dvec2(lerp(a.pos.x, b.pos.x), lerp(a.pos.y, b.pos.y)),
	                    size: dvec2(lerp(a.size.x, b.size.x), lerp(a.size.y, b.size.y)),
	                };

	                self.hero_from_draw_list.begin_always(cx);
	                let draw_list_id = self.hero_from_draw_list.id();
	                {
	                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
	                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
	                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
	                    dl.draw_list_uniforms.view_opacity = from_opacity;
	                }
	                for pair in &pairs {
	                    let r = lerp_rect(pair.from_rect, pair.to_rect);
	                    let hero = self.uid_to_widget(pair.from_uid);
	                    let _ = hero.draw_walk(cx, scope, Walk::abs_rect(r));
	                }
	                self.hero_from_draw_list.end(cx);

	                self.hero_to_draw_list.begin_always(cx);
	                let draw_list_id = self.hero_to_draw_list.id();
	                {
	                    let dl = &mut cx.cx.cx.draw_lists[draw_list_id];
	                    dl.draw_list_uniforms.view_shift = vec2(0.0, 0.0);
	                    dl.draw_list_uniforms.view_transform = Mat4f::identity();
	                    dl.draw_list_uniforms.view_opacity = to_opacity;
	                }
	                for pair in &pairs {
	                    let r = lerp_rect(pair.from_rect, pair.to_rect);
	                    let hero = self.uid_to_widget(pair.to_uid);
	                    let _ = hero.draw_walk(cx, scope, Walk::abs_rect(r));
	                }
	                self.hero_to_draw_list.end(cx);

                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            } else if hero_enabled {
                cx.global::<HeroGlobals>().set_phase(HeroPhase::Idle);
            }
        } else {
            let effect = Self::compute_effect(
                RouterTransitionPreset::None,
                RouterTransitionDirection::Forward,
                0.0,
                true,
                rect,
            );
		            Self::draw_route_into_draw_list(
		                cx,
		                scope,
		                &mut self.to_draw_list,
	                &mut self.route_widgets,
	                self.active_route,
	                effect,
	                false,
	            );
	        }

        if hero_enabled {
            cx.global::<HeroGlobals>().pop_router(router_uid);
        }

        self.draw_debug_inspector(cx, rect);

        cx.end_turtle_with_area(&mut self.area);
        DrawStep::done()
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawInspectorRect {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    color: Vec4f,
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

    pub fn navigate_with_transition(
        &self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate_with_transition(cx, route_id, transition)
        } else {
            false
        }
    }

    pub fn navigate_by_url(&self, cx: &mut Cx, url: &str) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.navigate_by_url(cx, url)
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

    pub fn back_with_transition(&self, cx: &mut Cx, transition: RouterTransitionSpec) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.back_with_transition(cx, transition)
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

    pub fn replace_with_transition(
        &self,
        cx: &mut Cx,
        route_id: LiveId,
        transition: RouterTransitionSpec,
    ) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.replace_with_transition(cx, route_id, transition)
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

    pub fn forward_with_transition(&self, cx: &mut Cx, transition: RouterTransitionSpec) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.forward_with_transition(cx, transition)
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

    pub fn current_url(&self) -> Option<String> {
        let inner = self.borrow()?;
        Some(inner.current_url())
    }

    pub fn current_route(&self) -> Option<Route> {
        if let Some(inner) = self.borrow() {
            inner.router.current_route().cloned()
        } else {
            None
        }
    }

    pub fn get_query_string(&self, key: &str) -> Option<String> {
        self.current_route()?.query_get_string(key)
    }

    pub fn get_query_i64(&self, key: &str) -> Option<i64> {
        self.current_route()?.query_get_i64(key)
    }

    pub fn get_query_u64(&self, key: &str) -> Option<u64> {
        self.current_route()?.query_get_u64(key)
    }

    pub fn get_query_bool(&self, key: &str) -> Option<bool> {
        self.current_route()?.query_get_bool(key)
    }

    pub fn get_query_f64(&self, key: &str) -> Option<f64> {
        self.current_route()?.query_get_f64(key)
    }

    pub fn get_state(&self) -> Option<RouterState> {
        Some(self.borrow()?.get_state())
    }

    pub fn set_state(&self, cx: &mut Cx, state: RouterState) -> bool {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_state(cx, state)
        } else {
            false
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
            .with_active_route_widget(|route_widget| {
                route_widget.button(&[button_id]).clicked(actions)
            })
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

    pub fn add_route_guard<F>(&self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterGuardDecision + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_route_guard(guard);
        }
    }

    pub fn add_route_guard_async<F>(&self, guard: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterGuardDecision>
            + Send
            + Sync
            + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_route_guard_async(guard);
        }
    }

    pub fn add_before_leave_hook<F>(&self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterBeforeLeaveDecision + Send + Sync + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_before_leave_hook(hook);
        }
    }

    pub fn add_before_leave_hook_async<F>(&self, hook: F)
    where
        F: Fn(&mut Cx, &RouterNavContext) -> RouterAsyncDecision<RouterBeforeLeaveDecision>
            + Send
            + Sync
            + 'static,
    {
        if let Some(mut inner) = self.borrow_mut() {
            inner.add_before_leave_hook_async(hook);
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
