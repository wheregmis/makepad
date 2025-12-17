use crate::{
    guards::{
        RouterAsyncDecision, RouterAsyncGuard, RouterBeforeLeaveAsync, RouterBeforeLeaveDecision,
        RouterBeforeLeaveSync, RouterGuardDecision, RouterNavContext, RouterSyncGuard,
    },
    route::Route,
    router::{Router, RouterAction},
    state::RouterState,
};
use makepad_draw::draw_list_2d::DrawListExt;
use makepad_widgets::*;

pub use crate::hero::Hero;

mod api;
mod actions;
mod guard_flow;
mod hero;
mod hero_render;
mod inspector;
mod live_apply;
mod nested;
mod path_nav;
mod route_widgets;
mod transitions;
mod url_sync;

use guard_flow::PendingNavigation;
use transitions::{RouterActionKind, RouterTransitionDirection, RouterTransitionState};
pub use transitions::{RouterTransitionPreset, RouterTransitionSpec};

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

impl RouterWidget {
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
        self.draw_routes_with_hero(cx, scope, rect);

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
