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
                    default_route: home
                    home = <HomePage> {}
                    settings = <SettingsPage> {}
                    about = <AboutPage> {}
                    user_profile = <UserProfilePage> {
                        route_pattern: "/user/:id"
                    }
                    admin = <AdminDashboard> {
                        route_pattern: "/admin/*"
                    }
                    not_found = <NotFoundPage> {
                        route_pattern: "/*"
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
    #[rust]
    callback_set: bool,
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

        // Set up route change callback once (if not already set)
        if !self.callback_set {
            router.on_route_change(|_cx, _old_route, new_route| {
                // Log route changes for debugging
                log!("Route changed to: {:?}", new_route.id);
            });
            self.callback_set = true;
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
        // Note: Child router should be auto-detected, but we still need to initialize it
        // when the admin route becomes active
        if router.current_route_id() == Some(live_id!(admin)) {
            let admin_router = self.ui.router_widget(ids!(router.admin.admin_router));

            // Initialize nested router on first access (auto-detection should handle registration)
            // But we still need to set the initial route
            if admin_router.current_route_id().is_none() {
                admin_router.navigate(cx, live_id!(admin_users));
            }

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

        // Display user ID from dynamic segment using the new helper method
        if router.current_route_id() == Some(live_id!(user_profile)) {
            // Use the new get_param_string helper method
            if let Some(user_id) = router.get_param_string("id") {
                // Only update if the user ID has changed
                if self.last_user_id.as_ref() != Some(&user_id) {
                    log!("User ID from route: {}", user_id);
                    // Update the label with the actual user ID
                    if let Some(mut label) = self
                        .ui
                        .label(ids!(router.user_profile.user_id_label))
                        .borrow_mut()
                    {
                        label.set_text(cx, &format!("User ID: {}", user_id));
                    }
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
