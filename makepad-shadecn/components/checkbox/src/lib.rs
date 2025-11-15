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
            margin: {left: 12, top: 0, right: 0, bottom: 0},
        }

        draw_bg: {
            size: 18.0,
            border_radius: 4.0,
            border_size: 1.5,

            color: #ffffff,
            color_hover: #f8fafc,
            color_down: #e2e8f0,
            color_active: #3b82f6,
            color_focus: vec4(0.145, 0.388, 0.922, 1.0),
            color_disabled: #f1f5f9,

            border_color: #e2e8f0,
            border_color_hover: #cbd5e1,
            border_color_down: #cbd5e1,
            border_color_active: #1d4ed8,
            border_color_focus: #93c5fd,
            border_color_disabled: #e2e8f099,

            mark_color: #ffffff00,
            mark_color_hover: #ffffff00,
            mark_color_down: #ffffff00,
            mark_color_active: #ffffff,
            mark_color_active_hover: #ffffff,
            mark_color_focus: #ffffff,
            mark_color_disabled: #ffffff55,
        }

        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #0f172a
        }
    }

    pub ShadecnCheckbox = <ShadecnCheckboxBase> {}
}
