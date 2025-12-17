use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub StackDemoPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x101A24 }
        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "History / Stack Demo"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "Exercises stack APIs: set_stack, pop_to, pop_to_root, replace."
            draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            set_stack_btn = <Button> { text: "Set stack: Home > Settings > About" }
            pop_to_settings_btn = <Button> { text: "Pop to Settings" }
            pop_to_root_btn = <Button> { text: "Pop to Root" }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            replace_about_btn = <Button> { text: "Replace → About" }
            home_btn = <Button> { text: "Back to Home" }
        }
    }
}
