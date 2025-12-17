use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub SettingsPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x0F3460 }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "Settings"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "This page toggles auth + dirty state to exercise guards and before-leave hooks."
            draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA }
        }

        auth_status_label = <Label> {
            text: "Auth: (unknown)"
            draw_text: { text_style: { font_size: 16 }, color: #xFFFFFF }
        }

        dirty_status_label = <Label> {
            text: "Dirty: (unknown)"
            draw_text: { text_style: { font_size: 16 }, color: #xFFFFFF }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            login_toggle_btn = <Button> { text: "Toggle Login" }
            dirty_toggle_btn = <Button> { text: "Toggle Dirty" }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            go_admin_btn = <Button> { text: "Go to /admin/dashboard (guarded)" }
            go_stack_demo_btn = <Button> { text: "Open stack demo" }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}
