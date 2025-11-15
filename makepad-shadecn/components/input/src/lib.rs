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
        padding: {left: 12, top: 10, right: 12, bottom: 10},
        empty_text: "",
        
        draw_bg: {
            border_radius: 6.0,
            border_size: 1.0,
            color: #ffffff,
            color_hover: #f8fafc,
            color_down: #f1f5f9,
            color_focus: #ffffff,
            color_empty: #f8fafc,
            color_disabled: #f1f5f9,
            border_color: #e2e8f0,
            border_color_hover: #cbd5e1,
            border_color_down: #cbd5e1,
            border_color_focus: #93c5fd,
            border_color_empty: #e2e8f0,
            border_color_disabled: #e2e8f080,
        }

        draw_text: {
            text_style: {
                font_size: 14.0,
            }
            color: #0f172a
        }

        draw_cursor: {
            color: vec4(0.145, 0.388, 0.922, 1.0)
        }

        draw_selection: {
            color: #bfdbfe
        }
    }

    pub ShadecnInput = <ShadecnInputBase> {}
}
