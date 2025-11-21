use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnSwitch = <CheckBoxFlat> {
        width: Fit,
        height: 24.0,
        padding: 0.0,

        icon_walk: {width: 0.0, height: 0.0}

        label_walk: {
            margin: {left: (44.0 + SPACE_2)}
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
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let w = 44.0;
                let h = 24.0;

                // Track
                sdf.box(0.0, 0.0, w, h, h * 0.5);

                let track_color = mix(
                    (COLOR_SLATE_200), // Unchecked (input color)
                    (COLOR_PRIMARY),     // Checked
                    self.active
                );

                sdf.fill(track_color);

                // Thumb
                let thumb_size = h - 4.0;
                let thumb_padding = 2.0;

                let thumb_x_off = thumb_padding;
                let thumb_x_on = w - thumb_size - thumb_padding;

                let thumb_x = mix(thumb_x_off, thumb_x_on, self.active);

                sdf.circle(thumb_x + thumb_size * 0.5, h * 0.5, thumb_size * 0.5);

                sdf.fill((COLOR_BG_PRIMARY));

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
                    from: {all: Forward {duration: 0.2}}
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
