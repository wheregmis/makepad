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
            admin_users = <AdminUsersPage> {}
            admin_settings = <AdminSettingsPage> {}
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

                    <View> { width: Fill, height: Fit }

                    back_btn = <Button> {
                        text: "← Back"
                    }
                }

                // Router content area
                router = <RouterWidget> {
                    width: Fill, height: Fill
                    home = <HomePage> {}
                    settings = <SettingsPage> {}
                    about = <AboutPage> {}
                    user_profile = <UserProfilePage> {}
                    admin = <AdminDashboard> {}
                    not_found = <NotFoundPage> {}
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
    router_initialized: bool,
    #[rust]
    admin_router_initialized: bool,
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

        // Initialize with home page on first load only
        if !self.router_initialized {
            log!("🚀 Initializing router with home page");

            // Register route patterns for dynamic segments and wildcards
            router
                .register_route_pattern("/user/:id", live_id!(user_profile))
                .unwrap();
            router
                .register_route_pattern("/admin/*", live_id!(admin))
                .unwrap();
            router
                .register_route_pattern("/*", live_id!(not_found))
                .unwrap(); // Catch-all wildcard

            router.navigate(cx, live_id!(home));
            self.router_initialized = true;
        }

        // Initialize nested admin router when admin route is active
        if router.current_route_id() == Some(live_id!(admin)) && !self.admin_router_initialized {
            // Register child router programmatically
            let admin_router_ref = self.ui.router_widget(ids!(router.admin.admin_router));
            router.register_child_router(live_id!(admin), admin_router_ref.clone());
            admin_router_ref.navigate(cx, live_id!(admin_users));
            self.admin_router_initialized = true;
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
            router.navigate_by_path(cx, "/admin/dashboard");
        }

        // Wildcard route example - navigate to non-existent route
        if self.ui.button(ids!(nav_bar.settings_btn)).clicked(&actions) && false {
            // This would trigger the wildcard route
            router.navigate_by_path(cx, "/nonexistent/route");
        }

        // Admin nested router navigation
        if router.current_route_id() == Some(live_id!(admin)) {
            let admin_router = self.ui.router_widget(ids!(router.admin.admin_router));

            if self
                .ui
                .button(ids!(
                    router.admin.admin_router.admin_users.admin_settings_btn
                ))
                .clicked(&actions)
            {
                log!("🔧 Admin: Navigating to settings (nested)");
                admin_router.navigate(cx, live_id!(admin_settings));
            }

            if self
                .ui
                .button(ids!(
                    router.admin.admin_router.admin_settings.admin_users_btn
                ))
                .clicked(&actions)
            {
                log!("👥 Admin: Navigating to users (nested)");
                admin_router.navigate(cx, live_id!(admin_users));
            }
        }

        // Display user ID from dynamic segment
        if router.current_route_id() == Some(live_id!(user_profile)) {
            if let Some(route) = router.current_route() {
                if let Some(user_id) = route.get_param(LiveId::from_str("id")) {
                    // Extract the user ID string from the LiveId
                    let user_id_str = user_id.as_string(|id_str| id_str.map(|s| s.to_string()));

                    if let Some(id) = user_id_str {
                        log!("User ID from route: {}", id);
                        // Update the label with the actual user ID
                        if let Some(mut label) = self
                            .ui
                            .label(ids!(router.user_profile.user_id_label))
                            .borrow_mut()
                        {
                            label.set_text(cx, &format!("User ID: {}", id));
                        }
                    }
                }
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
