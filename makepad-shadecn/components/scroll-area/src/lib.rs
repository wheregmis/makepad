use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnScrollArea = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 0,
        scroll_bars: <ScrollBars> {
            show_scroll_x: false,
            show_scroll_y: true,
            scroll_bar_x: <ScrollBar> {
                draw_bg: {
                    color: (COLOR_BORDER_SECONDARY),
                    color_hover: (COLOR_BORDER_PRIMARY),
                }
            }
            scroll_bar_y: <ScrollBar> {
                draw_bg: {
                    color: (COLOR_BORDER_SECONDARY),
                    color_hover: (COLOR_BORDER_PRIMARY),
                }
            }
        }
    }
}
