use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub HomePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x16213E }

        flow: Down, spacing: 18, padding: 40

        <Label> {
            text: "Router Demo App"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        <Label> {
            text: "A small Makepad app showcasing makepad-router features."
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Down, spacing: 10

            <Label> {
                text: "Highlights"
                draw_text: { text_style: { font_size: 14 }, color: #xFFFFFF }
            }
            <Label> { text: "• URL sync + deep links (web)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Nested routers (/admin/*)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Params + query (/user/:id?tab=...)" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Guards + before-leave hooks" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
            <Label> { text: "• Per-route transitions + hero transitions" draw_text: { text_style: { font_size: 12 }, color: #xAAAAAA } }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10

            settings_btn = <Button> { text: "Settings (guards)" }
            hero_btn = <Button> { text: "Hero transition" }
            user_btn = <Button> { text: "User (params/query)" }
            admin_btn = <Button> { text: "Admin (nested)" }
            stack_btn = <Button> { text: "History/Stack demo" }
            about_btn = <Button> { text: "About (async guard)" }
        }
    }
}
