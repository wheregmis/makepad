use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    // Horizontal separator — fills width, 1px tall by default
    pub ShadecnSeparator = <View> {
        width: Fill,
        height: 1.0,
        show_bg: true,
        draw_bg: {
            color: (COLOR_BORDER_PRIMARY),
        }
    }

    // Vertical separator — fills height, 1px wide
    pub ShadecnSeparatorVertical = <View> {
        width: 1.0,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (COLOR_BORDER_PRIMARY),
        }
    }
}
