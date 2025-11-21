use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCheckbox = <CheckBoxFlat> {
        width: Fit,
        height: Fit,
        padding: 0.0,

        icon_walk: {
            width: 0.0,
            height: 16.0,
            margin: 0.0
        }

        label_walk: {
            margin: {left: (16.0 + SPACE_2)}
        }

        draw_text: {
            text_style: {
                font_size: (FONT_SM),
            }
            color: (COLOR_FG_PRIMARY)
            color_hover: (COLOR_FG_PRIMARY)
            color_down: (COLOR_FG_PRIMARY)
            color_active: (COLOR_FG_PRIMARY)
            color_focus: (COLOR_FG_PRIMARY)
            color_disabled: (COLOR_FG_DISABLED)
        }

        draw_bg: {
            uniform border_radius: (RADIUS_SM)
            uniform border_width: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let sz = 16.0;
                let center_y = self.rect_size.y * 0.5;

                // Background
                sdf.box(
                    0.5,
                    center_y - sz * 0.5 + 0.5,
                    sz - 1.0,
                    sz - 1.0,
                    self.border_radius
                );

                let bg_color = mix(
                    (COLOR_BG_PRIMARY),
                    (COLOR_PRIMARY),
                    self.active
                );

                let border_color = mix(
                    (COLOR_PRIMARY),
                    (COLOR_PRIMARY),
                    self.active
                );

                sdf.fill_keep(bg_color);
                sdf.stroke(border_color, self.border_width);

                // Check mark
                let mark_padding = sz * 0.25;

                sdf.move_to(mark_padding, center_y);
                sdf.line_to(sz * 0.45, center_y + sz * 0.25);
                sdf.line_to(sz - mark_padding, center_y - sz * 0.25);

                let mark_color = (COLOR_PRIMARY_FOREGROUND);

                sdf.stroke(
                    mix(
                        (COLOR_WHITE_TRANSPARENT_00),
                        mark_color,
                        self.active
                    ),
                    1.5
                );

                return sdf.result
            }
        }

        animator: {
            hover = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {focus: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {focus: 1.0}
                    }
                }
            }
            active = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {active: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {active: 1.0}
                    }
                }
            }
        }
    }
}
