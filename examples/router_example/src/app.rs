use makepad_router::*;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;
    use makepad_router::widget::*;

    // Home Page
    HomePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x16213E
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Home Page"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "Welcome to the Router Example!"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10

            settings_btn = <Button> {
                text: "Go to Settings"
            }

            about_btn = <Button> {
                text: "Go to About"
            }

            user_btn = <Button> {
                text: "View User (Dynamic)"
            }

            hero_btn = <Button> {
                text: "Hero Demo"
            }

            admin_btn = <Button> {
                text: "Admin (Nested)"
            }
        }
    }

    // User Profile Page (Dynamic Segment Example)
    UserProfilePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x2D5016
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "User Profile"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        user_id_label = <Label> {
            text: "User ID: (dynamic)"
            draw_text: {
                text_style: { font_size: 18 }
                color: #xAAAAAA
            }
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    // 404 Not Found Page (Wildcard Example)
    NotFoundPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x4A0E4E
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "404 - Page Not Found"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "This is a catch-all wildcard route"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    // Admin Users Page (Nested Route) - Defined before AdminDashboard
    AdminUsersPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x2A3A4E
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Admin - Users"
            draw_text: {
                text_style: { font_size: 28 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "This is a nested route within Admin"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        admin_settings_btn = <Button> {
            text: "Go to Admin Settings"
        }
    }

    // Admin Settings Page (Nested Route) - Defined before AdminDashboard
    AdminSettingsPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x3A4A5E
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Admin - Settings"
            draw_text: {
                text_style: { font_size: 28 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "Another nested route within Admin"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        admin_users_btn = <Button> {
            text: "Go to Admin Users"
        }
    }

    // Admin Dashboard (Parent for Nested Router)
    AdminDashboard = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x1A1A3E
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Admin Dashboard"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "This page contains a nested router"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        // Nested router for admin sub-pages
        admin_router = <RouterWidget> {
            width: Fill, height: Fill
            default_route: admin_users
            not_found_route: admin_users
            push_transition: SharedAxis
            pop_transition: SharedAxis
            replace_transition: Fade
            transition_duration: 0.25
            admin_users = <AdminUsersPage> {
                route_pattern: "/dashboard"
            }
            admin_settings = <AdminSettingsPage> {
                route_pattern: "/settings"
            }
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    // Settings Page
    SettingsPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x0F3460
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Settings Page"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "Configure your preferences here"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    // About Page
    AboutPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: {
            color: #x533483
        }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "About Page"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "Router Example v1.0"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    // Hero Demo: shared element transition
    HeroListPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x13202A }
        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Hero Demo"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        <Label> {
            text: "Tap the card to transition"
            draw_text: {
                text_style: { font_size: 16 }
                color: #xAAAAAA
            }
        }

        hero_card = <Hero> {
            tag: hero_card
            width: 96, height: 96
            <View> {
                width: Fill, height: Fill
                show_bg: true
                draw_bg: { color: #xFFB000 }
            }
        }

        detail_btn = <Button> {
            text: "Open Detail"
        }

        home_btn = <Button> {
            text: "Back to Home"
        }
    }

    HeroDetailPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x0E1116 }
        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Hero Detail"
            draw_text: {
                text_style: { font_size: 32 }
                color: #xFFFFFF
            }
        }

        hero_card = <Hero> {
            tag: hero_card
            width: 280, height: 180
            <View> {
                width: Fill, height: Fill
                show_bg: true
                draw_bg: { color: #xFFB000 }
            }
        }

        back_btn = <Button> {
            text: "Back"
        }
    }

    // Main App
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

                // Navigation bar
                nav_bar = <View> {
                    width: Fill, height: Fit
                    show_bg: true
                    draw_bg: { color: #x252545 }
                    padding: 15
                    flow: Right, spacing: 10

                    home_btn = <Button> {
                        text: "Home"
                    }

                    settings_btn = <Button> {
                        text: "Settings"
                    }

                    about_btn = <Button> {
                        text: "About"
                    }

                    hero_btn = <Button> {
                        text: "Hero"
                    }

                    broken_link_btn = <Button> {
                        text: "Broken Link (404)"
                    }

                    <View> { width: Fill, height: Fit }

                    back_btn = <Button> {
                        text: "← Back"
                    }
                }

                // Router content area
                router = <RouterWidget> {
                    width: Fill, height: Fill
                    default_route: home
                    not_found_route: not_found
                    push_transition: SlideLeft
                    pop_transition: SlideRight
                    replace_transition: Fade
                    transition_duration: 0.30
                    hero_transition: true
                    home = <HomePage> {}
                    settings = <SettingsPage> {}
                    about = <AboutPage> {}
                    hero_list = <HeroListPage> {
                        route_pattern: "/hero"
                    }
                    hero_detail = <HeroDetailPage> {
                        route_pattern: "/hero/detail"
                    }
                    user_profile = <UserProfilePage> {
                        route_pattern: "/user/:id"
                    }
                    admin = <AdminDashboard> {
                        route_pattern: "/admin/*"
                    }
                    not_found = <NotFoundPage> {
                        route_transition: Fade
                        route_transition_duration: 0.20
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    last_user_id: Option<String>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_router::live_design(cx);
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let router = self.ui.router_widget(ids!(router));

        // Observe router changes without callbacks by consuming emitted RouterAction widget-actions.
        for action in actions.filter_widget_actions(router.widget_uid()) {
            if let Some(router_action) = action.action.downcast_ref::<RouterAction>() {
                if let RouterAction::RouteChanged { from, to } = router_action {
                    log!("Route changed: {:?} -> {:?}", from, to);
                }
            }
        }

        // Navigation bar buttons (outside router - use raw actions)
        if self.ui.button(ids!(nav_bar.home_btn)).clicked(&actions) {
            log!("🏠 Nav: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(nav_bar.settings_btn)).clicked(&actions) {
            log!("⚙️ Nav: Settings clicked");
            router.navigate(cx, live_id!(settings));
        }
        if self.ui.button(ids!(nav_bar.about_btn)).clicked(&actions) {
            log!("ℹ️ Nav: About clicked");
            router.navigate(cx, live_id!(about));
        }
        if self.ui.button(ids!(nav_bar.hero_btn)).clicked(&actions) {
            log!("🦸 Nav: Hero clicked");
            router.navigate(cx, live_id!(hero_list));
        }
        if self.ui.button(ids!(nav_bar.broken_link_btn)).clicked(&actions) {
            log!("🚫 Nav: Broken link clicked");
            router.navigate_by_path(cx, "/this/route/does/not/exist");
        }

        // Dynamic segment navigation example
        if self.ui.button(ids!(router.home.user_btn)).clicked(&actions) {
            log!("👤 Navigating to user profile with dynamic segment");
            router.navigate_by_path(cx, "/user/12345");
        }

        // Nested router navigation example
        if self
            .ui
            .button(ids!(router.home.admin_btn))
            .clicked(&actions)
        {
            log!("🔐 Navigating to admin (nested router)");
            // Demonstrate nested routing: `/admin/*` activates the Admin route and delegates the tail
            // (e.g. `/dashboard`) into `admin_router` based on its own route patterns.
            //
            // We clear history afterwards so the navbar back button cannot return to Home from Admin.
            router.navigate_by_path(cx, "/admin/dashboard");
            router.clear_history(cx);
        }

        // Admin nested router navigation
        // Note: Child router should be auto-detected, but we still need to initialize it
        // when the admin route becomes active
        if router.current_route_id() == Some(live_id!(admin)) {
            let admin_router = self.ui.router_widget(ids!(router.admin.admin_router));

            for action in actions.filter_widget_actions(admin_router.widget_uid()) {
                if let Some(router_action) = action.action.downcast_ref::<RouterAction>() {
                    if let RouterAction::RouteChanged { from, to } = router_action {
                        log!("Admin route changed: {:?} -> {:?}", from, to);
                    }
                }
            }

            if self
                .ui
                .button(ids!(
                    router.admin.admin_router.admin_users.admin_settings_btn
                ))
                .clicked(&actions)
            {
                log!("🔧 Admin: Navigating to settings (nested)");
                admin_router.push(cx, live_id!(admin_settings));
            }

            if self
                .ui
                .button(ids!(
                    router.admin.admin_router.admin_settings.admin_users_btn
                ))
                .clicked(&actions)
            {
                log!("👥 Admin: Navigating to users (nested)");
                admin_router.push(cx, live_id!(admin_users));
            }
        }

        // Display user ID from dynamic segment using the new helper method
        if router.current_route_id() == Some(live_id!(user_profile)) {
            if let Some(user_id) = router.get_param_string("id") {
                if self.last_user_id.as_ref() != Some(&user_id) {
                    log!("User ID from route: {}", user_id);
                    router.bind_param_to_label(cx, "id", live_id!(user_id_label), |id| {
                        format!("User ID: {}", id)
                    });
                    self.last_user_id = Some(user_id);
                }
            }
        } else {
            // Clear the last user ID when not on user profile page
            if self.last_user_id.is_some() {
                self.last_user_id = None;
            }
        }

        // Back button
        if self.ui.button(ids!(nav_bar.back_btn)).clicked(&actions) {
            log!("⬅️ Nav: Back clicked");
            router.back(cx);
        }

        // Routed buttons now generate actions even when inactive
        if self
            .ui
            .button(ids!(router.home.settings_btn))
            .clicked(&actions)
        {
            log!("🏠→⚙️ Home: Settings clicked");
            router.navigate(cx, live_id!(settings));
        }
        if self
            .ui
            .button(ids!(router.home.about_btn))
            .clicked(&actions)
        {
            log!("🏠→ℹ️ Home: About clicked");
            router.navigate(cx, live_id!(about));
        }
        if self.ui.button(ids!(router.home.hero_btn)).clicked(&actions) {
            log!("🏠→🦸 Home: Hero clicked");
            router.navigate(cx, live_id!(hero_list));
        }
        if self.ui.button(ids!(router.hero_list.detail_btn)).clicked(&actions) {
            log!("🦸 Hero: Open Detail clicked");
            router.navigate(cx, live_id!(hero_detail));
        }
        if self.ui.button(ids!(router.hero_list.home_btn)).clicked(&actions) {
            log!("🦸→🏠 Hero: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(router.hero_detail.back_btn)).clicked(&actions) {
            log!("🦸 Detail: Back clicked");
            router.back(cx);
        }
        if self
            .ui
            .button(ids!(router.settings.home_btn))
            .clicked(&actions)
        {
            log!("⚙️→🏠 Settings: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self
            .ui
            .button(ids!(router.about.home_btn))
            .clicked(&actions)
        {
            log!("ℹ️→🏠 About: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self
            .ui
            .button(ids!(router.admin.home_btn))
            .clicked(&actions)
        {
            log!("🔐→🏠 Admin: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self
            .ui
            .button(ids!(router.user_profile.home_btn))
            .clicked(&actions)
        {
            log!("👤→🏠 User Profile: Home clicked");
            router.navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(router.not_found.home_btn)).clicked(&actions) {
            log!("🚫→🏠 404: Back to Home clicked");
            router.replace(cx, live_id!(home));
        }

        // Update back button state
        self.ui
            .button(ids!(nav_bar.back_btn))
            .set_enabled(cx, router.can_go_back());
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
