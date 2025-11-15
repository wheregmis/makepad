use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnRadioButtonBase = <RadioButtonFlat> {
        draw_bg: {
            instance down: 0.0,
            instance disabled: 0.0,

            size: 16.0,
            border_size: 1.5,
            border_radius: (RADIUS_FULL),

            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_HOVER),
            color_active: (COLOR_BG_PRIMARY),
            color_focus: (COLOR_BG_PRIMARY),
            color_disabled: (COLOR_BG_DISABLED),

            border_color: (COLOR_FG_TERTIARY),
            border_color_hover: (COLOR_PRIMARY),
            border_color_active: (COLOR_PRIMARY),
            border_color_focus: (COLOR_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),

            mark_color: (COLOR_BG_PRIMARY),
            mark_color_active: (COLOR_PRIMARY),
            mark_color_disabled: (COLOR_FG_DISABLED),
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY),
            color_hover: (COLOR_FG_PRIMARY),
            color_down: (COLOR_FG_PRIMARY),
            color_active: (COLOR_FG_PRIMARY),
            color_focus: (COLOR_FG_PRIMARY),
            color_disabled: (COLOR_FG_DISABLED),
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 0.0}], hover: 0.0}
                        draw_text: {down: [{time: 0.0, value: 0.0}], hover: 0.0}
                    }
                }
                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 0.0}], hover: [{time: 0.0, value: 1.0}]}
                        draw_text: {down: [{time: 0.0, value: 0.0}], hover: [{time: 0.0, value: 1.0}]}
                    }
                }
                down = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                        draw_text: {down: [{time: 0.0, value: 1.0}], hover: 1.0}
                    }
                }
            }
        }
    }

    pub ShadecnRadioButton = <ShadecnRadioButtonBase> {}

    pub ShadecnRadioButtonGroup = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_2),
    }
}
