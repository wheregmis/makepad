use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnInputBase = <TextInputFlat> {
        width: Fill,
        height: Fit,
        padding: {left: (SPACE_2), top: (SPACE_3), right: (SPACE_2), bottom: (SPACE_3)},

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: 0.0, hover: 0.0}
                        draw_text: {down: 0.0, hover: 0.0}
                    }
                }
                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        down: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {down: 0.0, hover: [{time: 0.0, value: 1.0}]}
                        draw_text: {down: 0.0, hover: [{time: 0.0, value: 1.0}]}
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

        draw_bg: {
            instance hover: 0.0,
            instance focus: 0.0,
            instance disabled: 0.0,

            border_size: 1.0,
            border_radius: (RADIUS_MD),

            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_PRIMARY),
            color_focus: (COLOR_BG_PRIMARY),
            color_disabled: (COLOR_BG_DISABLED),
            color_empty: (COLOR_BG_PRIMARY),

            border_color: (COLOR_BORDER_PRIMARY),
            border_color_hover: (COLOR_BORDER_HOVER),
            border_color_focus: (COLOR_PRIMARY),
            border_color_disabled: (COLOR_BORDER_DISABLED),

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - (self.border_size * 2.0),
                    self.rect_size.y - (self.border_size * 2.0),
                    max(1.0, self.border_radius)
                );

                let bg_color = mix(
                    mix(
                        mix(self.color, self.color_focus, self.focus),
                        self.color_hover,
                        self.hover
                    ),
                    self.color_disabled,
                    self.disabled
                );

                sdf.fill_keep(bg_color);

                let border_color = mix(
                    mix(
                        mix(self.border_color, self.border_color_focus, self.focus),
                        self.border_color_hover,
                        self.hover
                    ),
                    self.border_color_disabled,
                    self.disabled
                );

                if self.border_size > 0.0 {
                    sdf.stroke(border_color, self.border_size);
                }

                return sdf.result;
            }
        }

        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }

            color: (COLOR_FG_PRIMARY),
            color_hover: (COLOR_FG_PRIMARY),
            color_focus: (COLOR_FG_PRIMARY),
            color_disabled: (COLOR_FG_DISABLED),
            color_empty: (COLOR_FG_TERTIARY),
        }

        draw_selection: {
            color: (COLOR_ACCENT_LIGHT),
            color_focus: (COLOR_ACCENT_LIGHT),
        }

        draw_cursor: {
            color: (COLOR_PRIMARY),
        }
    }

    pub ShadecnInput = <ShadecnInputBase> {}
}
