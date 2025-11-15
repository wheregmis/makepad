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
            border_radius: 22.0,
            border_size: 0.0,

            color: #e2e8f0,
            color_hover: #cbd5e1,
            color_down: #cbd5e1,
            color_active: #3b82f6,
            color_focus: #3b82f6,
            color_disabled: #e2e8f0,

            border_color: #e2e8f0,
            border_color_hover: #cbd5e1,
            border_color_down: #cbd5e1,
            border_color_active: #1d4ed8,
            border_color_focus: #93c5fd,
            border_color_disabled: #e2e8f099,

            // Toggle thumb (the circle that moves)
            mark_size: 0.65,
            mark_color: #ffffff,
            mark_color_hover: #ffffff,
            mark_color_down: #ffffff,
            mark_color_active: #ffffff,
            mark_color_active_hover: #ffffff,
            mark_color_focus: #ffffff,
            mark_color_disabled: #ffffffaa,
        }

        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #0f172a
        }
    }

    pub ShadecnToggle = <ShadecnToggleBase> {}
}

