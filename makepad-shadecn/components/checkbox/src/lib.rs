use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCheckboxBase = <CheckBoxFlat> {
        width: Fit,
        height: Fit,
        margin: {left: 0, top: 0, right: 0, bottom: 0},
        padding: {left: 0, top: 2, right: 0, bottom: 2},

        label_walk: {
            width: Fit, height: Fit,
            margin: {left: (SPACE_3), top: 0, right: 0, bottom: 0},
        }

        draw_bg: {
            size: 16.0,
            border_radius: (RADIUS_SM),
            border_size: 1.5,

            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_HOVER),
            color_down: (COLOR_BG_ACTIVE),
            color_active: (COLOR_ACCENT),
            color_focus: (COLOR_ACCENT_HOVER),
            color_disabled: (COLOR_BG_DISABLED),

            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_down: (COLOR_BORDER_SECONDARY),
            border_color_active: (COLOR_ACCENT_DARK),
            border_color_focus: (COLOR_ACCENT_FOCUS),
            border_color_disabled: (COLOR_BORDER_DISABLED),

            mark_color: (COLOR_WHITE_TRANSPARENT_00),
            mark_color_hover: (COLOR_WHITE_TRANSPARENT_00),
            mark_color_down: (COLOR_WHITE_TRANSPARENT_00),
            mark_color_active: (COLOR_WHITE),
            mark_color_active_hover: (COLOR_WHITE),
            mark_color_focus: (COLOR_WHITE),
            mark_color_disabled: (COLOR_WHITE_TRANSPARENT_55),
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnCheckbox = <ShadecnCheckboxBase> {}
}
