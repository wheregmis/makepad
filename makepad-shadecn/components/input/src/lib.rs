use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnInputBase = <TextInputFlat> {
        width: Fill,
        height: Fit,
        margin: {left: 0, top: 0, right: 0, bottom: 0},
        padding: {left: (SPACE_3), top: 10, right: (SPACE_3), bottom: 10},
        empty_text: "",
        
        draw_bg: {
            border_radius: (RADIUS_MD),
            border_size: 1.0,
            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_SECONDARY),
            color_down: (COLOR_BG_HOVER),
            color_focus: (COLOR_BG_PRIMARY),
            color_empty: (COLOR_BG_SECONDARY),
            color_disabled: (COLOR_BG_DISABLED),
            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_down: (COLOR_BORDER_SECONDARY),
            border_color_focus: (COLOR_ACCENT_FOCUS),
            border_color_empty: (COLOR_BORDER_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY)
        }

        draw_cursor: {
            color: (COLOR_ACCENT_HOVER)
        }

        draw_selection: {
            color: (COLOR_ACCENT_LIGHTER)
        }
    }

    pub ShadecnInput = <ShadecnInputBase> {}
}
