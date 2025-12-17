mod pages;
live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;
    use makepad_draw::shader::std::*;
    use crate::app::pages::home::*;
    use crate::app::pages::settings::*;
    use crate::app::pages::about::*;
    use crate::app::pages::stack_demo::*;
    use crate::app::pages::hero::*;
    use crate::app::pages::user_profile::*;
    use crate::app::pages::admin::*;
    use crate::app::pages::not_found::*;

    App = {{App}} {
        ui: <Window> {
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return mix(#x1A1A2E, #x0F0F1E, self.pos.y);
                }
            }
            width: Fill, height: Fill

            body = <View> {
                width: Fill, height: Fill
                flow: Down

                nav_bar = <View> {
                    width: Fill, height: Fit
                    show_bg: true
                    draw_bg: { color: #x252545 }
                    padding: 15
                    flow: Right, spacing: 10

                    home_btn = <Button> { text: "Home" }
                    settings_btn = <Button> { text: "Settings" }
                    hero_btn = <Button> { text: "Hero" }
                    admin_btn = <Button> { text: "Admin" }
                    stack_btn = <Button> { text: "Stack" }
                    broken_link_btn = <Button> { text: "404" }

                    <View> { width: Fill, height: Fit }

                    status_label = <Label> {
                        text: ""
                        draw_text: { text_style: { font_size: 10 }, color: #xAAAAAA }
                    }

                    back_btn = <Button> { text: "← Back" }
                }

                router = <RouterWidget> {
                    width: Fill, height: Fill
                    default_route: home
                    not_found_route: not_found
                    push_transition: SlideLeft
                    pop_transition: SlideRight
                    replace_transition: Fade
                    transition_duration: 0.30
                    hero_transition: true
                    debug_inspector: true
                    use_initial_url: true

                    home = <HomePage> { route_pattern: "/" }
                    settings = <SettingsPage> { route_pattern: "/settings" }
                    about = <AboutPage> { route_pattern: "/about" }
                    stack_demo = <StackDemoPage> { route_pattern: "/stack" }

                    hero_list = <HeroListPage> { route_pattern: "/hero" }
                    hero_detail = <HeroDetailPage> { route_pattern: "/hero/detail" }

                    user_profile = <UserProfilePage> { route_pattern: "/user/:id" }

                    admin = <AdminDashboard> { route_pattern: "/admin/*" }

                    not_found = <NotFoundPage> {
                        route_transition: Fade
                        route_transition_duration: 0.20
                    }
                }
            }
        }
    }
}

use makepad_router::{
    Route, RouterAction, RouterAsyncDecision, RouterBeforeLeaveDecision, RouterGuardDecision,
    RouterRedirect, RouterRedirectTarget, RouterWidgetWidgetRefExt,
};
use makepad_widgets::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    last_user_id: Option<String>,
    #[rust]
    last_user_tab: Option<String>,
    #[rust]
    auth_logged_in: Arc<AtomicBool>,
    #[rust]
    settings_dirty: Arc<AtomicBool>,
    #[rust]
    hooks_installed: bool,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_router::live_design(cx);
        makepad_widgets::live_design(cx);
        pages::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(ids!(router));

        // Observe router changes by consuming emitted RouterAction widget-actions.
        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(router_action) = action.action.downcast_ref::<RouterAction>() {
                if let RouterAction::RouteChanged { from, to } = router_action {
                    log!("Route changed: {:?} -> {:?}", from, to);
                }
            }
        }

        // Nav bar
        if self.ui.button(ids!(nav_bar.home_btn)).clicked(actions) {
            router.navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(nav_bar.settings_btn)).clicked(actions) {
            router.navigate(cx, live_id!(settings));
        }
        if self.ui.button(ids!(nav_bar.hero_btn)).clicked(actions) {
            router.navigate(cx, live_id!(hero_list));
        }
        if self.ui.button(ids!(nav_bar.admin_btn)).clicked(actions) {
            router.navigate_by_path(cx, "/admin/dashboard");
        }
        if self.ui.button(ids!(nav_bar.stack_btn)).clicked(actions) {
            router.navigate(cx, live_id!(stack_demo));
        }
        if self.ui.button(ids!(nav_bar.broken_link_btn)).clicked(actions) {
            router.navigate_by_path(cx, "/this/route/does/not/exist");
        }
        if self.ui.button(ids!(nav_bar.back_btn)).clicked(actions) {
            router.back(cx);
        }

        // Routed buttons: only check the active route for best performance.
        match router.current_route_id() {
            Some(route_id) if route_id == live_id!(home) => {
                if self.ui.button(ids!(router.home.settings_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(settings));
                }
                if self.ui.button(ids!(router.home.hero_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(hero_list));
                }
                if self.ui.button(ids!(router.home.user_btn)).clicked(actions) {
                    router.navigate_by_path(cx, "/user/12345?tab=posts");
                }
                if self.ui.button(ids!(router.home.admin_btn)).clicked(actions) {
                    router.navigate_by_path(cx, "/admin/dashboard");
                }
                if self.ui.button(ids!(router.home.stack_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(stack_demo));
                }
                if self.ui.button(ids!(router.home.about_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(about));
                }
            }
            Some(route_id) if route_id == live_id!(settings) => {
                if self.ui.button(ids!(router.settings.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
                if self.ui.button(ids!(router.settings.login_toggle_btn)).clicked(actions) {
                    let next = !self.auth_logged_in.load(Ordering::SeqCst);
                    self.auth_logged_in.store(next, Ordering::SeqCst);
                }
                if self.ui.button(ids!(router.settings.dirty_toggle_btn)).clicked(actions) {
                    let next = !self.settings_dirty.load(Ordering::SeqCst);
                    self.settings_dirty.store(next, Ordering::SeqCst);
                }
                if self.ui.button(ids!(router.settings.go_admin_btn)).clicked(actions) {
                    router.navigate_by_path(cx, "/admin/dashboard");
                }
                if self.ui.button(ids!(router.settings.go_stack_demo_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(stack_demo));
                }
            }
            Some(route_id) if route_id == live_id!(about) => {
                if self.ui.button(ids!(router.about.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
            }
            Some(route_id) if route_id == live_id!(stack_demo) => {
                if self.ui.button(ids!(router.stack_demo.set_stack_btn)).clicked(actions) {
                    router.set_stack(
                        cx,
                        vec![Route::new(live_id!(home)), Route::new(live_id!(settings)), Route::new(live_id!(about))],
                    );
                }
                if self.ui.button(ids!(router.stack_demo.pop_to_settings_btn)).clicked(actions) {
                    router.pop_to(cx, live_id!(settings));
                }
                if self.ui.button(ids!(router.stack_demo.pop_to_root_btn)).clicked(actions) {
                    router.pop_to_root(cx);
                }
                if self.ui.button(ids!(router.stack_demo.replace_about_btn)).clicked(actions) {
                    router.replace(cx, live_id!(about));
                }
                if self.ui.button(ids!(router.stack_demo.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
            }
            Some(route_id) if route_id == live_id!(hero_list) => {
                if self.ui.button(ids!(router.hero_list.detail_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(hero_detail));
                }
                if self.ui.button(ids!(router.hero_list.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
            }
            Some(route_id) if route_id == live_id!(hero_detail) => {
                if self.ui.button(ids!(router.hero_detail.back_btn)).clicked(actions) {
                    router.back(cx);
                }
            }
            Some(route_id) if route_id == live_id!(user_profile) => {
                if let Some(user_id) = router.get_param_string("id") {
                    if self.last_user_id.as_ref() != Some(&user_id) {
                        router.bind_param_to_label(cx, "id", live_id!(user_id_label), |id| {
                            format!("User ID: {}", id)
                        });
                        self.last_user_id = Some(user_id);
                    }
                }

                let tab = router
                    .get_query_string("tab")
                    .unwrap_or_else(|| "(none)".to_string());
                if self.last_user_tab.as_ref() != Some(&tab) {
                    self.ui
                        .label(ids!(router.user_profile.tab_label))
                        .set_text(cx, &format!("Query tab: {}", tab));
                    self.last_user_tab = Some(tab);
                }

                if self.ui.button(ids!(router.user_profile.tab_posts_btn)).clicked(actions) {
                    router.navigate_by_path(cx, "/user/12345?tab=posts");
                }
                if self.ui.button(ids!(router.user_profile.tab_likes_btn)).clicked(actions) {
                    router.navigate_by_path(cx, "/user/12345?tab=likes");
                }
                if self.ui.button(ids!(router.user_profile.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
            }
            Some(route_id) if route_id == live_id!(admin) => {
                // Handle nested router buttons
                let admin_router = self.ui.router_widget(ids!(router.admin.admin_router));
                match admin_router.current_route_id() {
                    Some(admin_route) if admin_route == live_id!(admin_users) => {
                        if self
                            .ui
                            .button(ids!(router.admin.admin_router.admin_users.admin_settings_btn))
                            .clicked(actions)
                        {
                            admin_router.navigate_by_path(cx, "/settings");
                        }
                        if self
                            .ui
                            .button(ids!(router.admin.admin_router.admin_users.admin_user_42_btn))
                            .clicked(actions)
                        {
                            admin_router.navigate_by_path(cx, "/users/42");
                        }
                    }
                    Some(admin_route) if admin_route == live_id!(admin_settings) => {
                        if self
                            .ui
                            .button(ids!(router.admin.admin_router.admin_settings.admin_users_btn))
                            .clicked(actions)
                        {
                            admin_router.navigate_by_path(cx, "/dashboard");
                        }
                    }
                    Some(admin_route) if admin_route == live_id!(admin_user_detail) => {
                        if let Some(user_id) = admin_router.get_param_string("id") {
                            self.ui
                                .label(ids!(router.admin.admin_router.admin_user_detail.admin_user_id_label))
                                .set_text(cx, &format!("User ID: {}", user_id));
                        }
                        if self
                            .ui
                            .button(ids!(router.admin.admin_router.admin_user_detail.back_btn))
                            .clicked(actions)
                        {
                            admin_router.back(cx);
                        }
                    }
                    _ => {}
                }

                if self.ui.button(ids!(router.admin.home_btn)).clicked(actions) {
                    router.navigate(cx, live_id!(home));
                }
            }
            Some(route_id) if route_id == live_id!(not_found) => {
                if self.ui.button(ids!(router.not_found.home_btn)).clicked(actions) {
                    router.replace(cx, live_id!(home));
                }
            }
            _ => {
                self.last_user_id = None;
                self.last_user_tab = None;
            }
        }

        // Update back button state + status label.
        self.ui
            .button(ids!(nav_bar.back_btn))
            .set_enabled(cx, router.can_go_back());
        let status = format!(
            "{}  {}",
            router
                .current_route_id()
                .map(|id: LiveId| id.to_string())
                .unwrap_or_else(|| "(none)".to_string()),
            router.current_url().unwrap_or_else(|| "(no url)".to_string())
        );
        self.ui.label(ids!(nav_bar.status_label)).set_text(cx, &status);

        if router.current_route_id() == Some(live_id!(settings)) {
            self.ui
                .label(ids!(router.settings.auth_status_label))
                .set_text(
                    cx,
                    if self.auth_logged_in.load(Ordering::SeqCst) {
                        "Auth: logged in"
                    } else {
                        "Auth: logged out (admin is guarded)"
                    },
                );
            self.ui
                .label(ids!(router.settings.dirty_status_label))
                .set_text(
                    cx,
                    if self.settings_dirty.load(Ordering::SeqCst) {
                        "Dirty: true (before-leave blocks leaving Settings)"
                    } else {
                        "Dirty: false"
                    },
                );
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if matches!(event, Event::Startup) && !self.hooks_installed {
            self.hooks_installed = true;
            self.install_router_hooks(cx);
        }
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl App {
    fn install_router_hooks(&mut self, _cx: &mut Cx) {
        let router = self.ui.router_widget(ids!(router));
        let auth = self.auth_logged_in.clone();
        let dirty = self.settings_dirty.clone();

        // Guard: admin requires auth; redirect to settings.
        router.add_route_guard(move |_cx, nav| {
            let to = nav.to.as_ref().map(|r| r.id);
            if to == Some(live_id!(admin)) && !auth.load(Ordering::SeqCst) {
                return RouterGuardDecision::Redirect(RouterRedirect {
                    target: RouterRedirectTarget::Route(live_id!(settings)),
                    replace: true,
                });
            }
            RouterGuardDecision::Allow
        });

        // Before leave: block leaving Settings when dirty.
        router.add_before_leave_hook(move |_cx, nav| {
            let from = nav.from.as_ref().map(|r| r.id);
            let to = nav.to.as_ref().map(|r| r.id);
            if from == Some(live_id!(settings))
                && to != Some(live_id!(settings))
                && dirty.load(Ordering::SeqCst)
            {
                return RouterBeforeLeaveDecision::Block;
            }
            RouterBeforeLeaveDecision::Allow
        });

        // Async guard: about is delayed on native to demo async decisions.
        router.add_route_guard_async(move |_cx, nav| {
            if nav.to.as_ref().map(|r| r.id) != Some(live_id!(about)) {
                return RouterAsyncDecision::Immediate(RouterGuardDecision::Allow);
            }

            #[cfg(target_arch = "wasm32")]
            {
                RouterAsyncDecision::Immediate(RouterGuardDecision::Allow)
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let rx: ToUIReceiver<RouterGuardDecision> = Default::default();
                let tx = rx.sender();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(200));
                    let _ = tx.send(RouterGuardDecision::Allow);
                });
                RouterAsyncDecision::Pending(rx)
            }
        });
    }
}

app_main!(App);
