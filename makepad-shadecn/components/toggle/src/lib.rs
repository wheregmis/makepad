use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use makepad_draw::shader::std::*;

    pub ShadecnToggleBase = <CheckBoxFlat> {
        width: Fit,
        height: Fit,
        margin: {left: 0, top: 0, right: 0, bottom: 0},
        padding: {left: 0, top: 2, right: 0, bottom: 2},

        label_walk: {
            width: 0, height: 0,
            margin: {left: 0, top: 0, right: 0, bottom: 0},
        }

        draw_bg: {
            size: 44.0,
            check_type: Toggle,
            border_radius: (RADIUS_FULL),
            border_size: 0.0,

            color: (COLOR_SLATE_200),
            color_hover: (COLOR_BORDER_HOVER),
            color_down: (COLOR_BORDER_SECONDARY),
            color_active: (COLOR_ACCENT),
            color_focus: (COLOR_ACCENT),
            color_disabled: (COLOR_SLATE_200),

            border_color: (COLOR_SLATE_200),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_down: (COLOR_BORDER_SECONDARY),
            border_color_active: (COLOR_ACCENT_DARK),
            border_color_focus: (COLOR_ACCENT_FOCUS),
            border_color_disabled: (COLOR_BORDER_DISABLED),

            // Toggle thumb (the circle that moves)
            mark_size: 0.65,
            mark_color: (COLOR_WHITE),
            mark_color_hover: (COLOR_WHITE),
            mark_color_down: (COLOR_WHITE),
            mark_color_active: (COLOR_WHITE),
            mark_color_active_hover: (COLOR_WHITE),
            mark_color_focus: (COLOR_WHITE),
            mark_color_disabled: (COLOR_WHITE_TRANSPARENT_AA),
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnToggle = <ShadecnToggleBase> {}
}

