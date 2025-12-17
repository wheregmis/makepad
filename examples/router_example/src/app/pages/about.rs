use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub AboutPage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x533483 }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "About"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "This route is behind an async guard (native adds a small delay)."
            draw_text: { text_style: { font_size: 14 }, color: #xAAAAAA }
        }

        <Label> {
            text: "Router demo app"
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}
