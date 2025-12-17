use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use link::theme_desktop_dark::*;

    pub UserProfilePage = <View> {
        width: Fill, height: Fill
        show_bg: true
        draw_bg: { color: #x2D5016 }

        flow: Down, spacing: 20, padding: 40

        <Label> {
            text: "User Profile"
            draw_text: { text_style: { font_size: 32 }, color: #xFFFFFF }
        }

        user_id_label = <Label> {
            text: "User ID: (dynamic)"
            draw_text: { text_style: { font_size: 18 }, color: #xAAAAAA }
        }

        tab_label = <Label> {
            text: "Query tab: (none)"
            draw_text: { text_style: { font_size: 16 }, color: #xAAAAAA }
        }

        <View> {
            width: Fill, height: Fit
            flow: Right, spacing: 10
            tab_posts_btn = <Button> { text: "tab=posts" }
            tab_likes_btn = <Button> { text: "tab=likes" }
        }

        home_btn = <Button> { text: "Back to Home" }
    }
}
