use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnSwitch = <CheckBoxFlat> {
        width: Fit,
        height: 22.0,
        padding: 0.0,

        icon_walk: {width: 0.0, height: 0.0}

        label_walk: {
            margin: {left: (SPACE_3 + 42.0)}
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

                let w = 42.0;
                let h = 22.0;

                // Track
                let track_radius = h * 0.5;
                let track_border = 1.0;
                sdf.box(track_border, track_border, w - track_border * 2.0, h - track_border * 2.0, track_radius);

                let base_off = COLOR_SLATE_200;
                let base_on = COLOR_PRIMARY;
                let base_disabled = COLOR_BG_DISABLED;
                let track_color_enabled = mix(base_off, base_on, self.active);
                let track_color = mix(track_color_enabled, base_disabled, self.disabled);

                sdf.fill_keep(track_color);
                sdf.stroke(COLOR_BORDER_PRIMARY * vec4(1.0, 1.0, 1.0, 0.35), track_border);

                // Thumb
                let thumb_size = h - 8.0;
                let thumb_padding = 4.0;

                let thumb_x_off = thumb_padding;
                let thumb_x_on = w - thumb_size - thumb_padding;
                let thumb_x = mix(thumb_x_off, thumb_x_on, self.active);

                sdf.circle(thumb_x + thumb_size * 0.5, h * 0.5, thumb_size * 0.5);
                let thumb_color = mix(COLOR_BG_PRIMARY, COLOR_BG_SECONDARY, self.disabled);
                sdf.fill_keep(thumb_color);
                sdf.stroke(COLOR_BORDER_PRIMARY * vec4(1.0, 1.0, 1.0, 0.45), 1.0);

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
