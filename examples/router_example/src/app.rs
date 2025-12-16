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
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_router::live_design(cx);
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Navigation bar buttons
        if self.ui.button(ids!(nav_bar.home_btn)).clicked(&actions) {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(home));
        }
        if self.ui.button(ids!(nav_bar.settings_btn)).clicked(&actions) {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(settings));
        }
        if self.ui.button(ids!(nav_bar.about_btn)).clicked(&actions) {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(about));
        }

        // Back button
        if self.ui.button(ids!(nav_bar.back_btn)).clicked(&actions) {
            self.ui.router_widget(ids!(router)).back(cx);
        }

        // Home page buttons
        if self
            .ui
            .button(ids!(router.home.settings_btn))
            .clicked(&actions)
        {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(settings));
        }
        if self
            .ui
            .button(ids!(router.home.about_btn))
            .clicked(&actions)
        {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(about));
        }

        // Settings page button
        if self
            .ui
            .button(ids!(router.settings.home_btn))
            .clicked(&actions)
        {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(home));
        }

        // About page button
        if self
            .ui
            .button(ids!(router.about.home_btn))
            .clicked(&actions)
        {
            self.ui
                .router_widget(ids!(router))
                .navigate(cx, live_id!(home));
        }

        // Update back button state
        let router = self.ui.router_widget(ids!(router));
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
