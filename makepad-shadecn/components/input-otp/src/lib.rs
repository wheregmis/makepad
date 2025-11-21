use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnInputOTPSlot = <TextInputFlat> {
        width: 40.0, height: 50.0,
        padding: {left: 0.0, right: 0.0, top: 12.0, bottom: 0.0}
        label_align: {x: 0.5, y: 0.0}
        
        is_numeric_only: true
        empty_text: ""
        
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance disabled: 0.0

            border_size: 1.0,
            border_radius: (RADIUS_MD),

            color: (COLOR_BG_PRIMARY),
            color_hover: (COLOR_BG_PRIMARY),
            color_focus: (COLOR_BG_PRIMARY),
            color_disabled: (COLOR_BG_DISABLED),

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
                font_size: (FONT_LG),
            }
            color: (COLOR_FG_PRIMARY),
        }
        
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
            }
            focus = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.2}}
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
        }
    }

    pub ShadecnInputOTP = <View> {
        width: Fit, height: Fit,
        flow: Right,
        spacing: (SPACE_2),
        
        slot1 = <ShadecnInputOTPSlot> {}
        slot2 = <ShadecnInputOTPSlot> {}
        slot3 = <ShadecnInputOTPSlot> {}
        slot4 = <ShadecnInputOTPSlot> {}
        slot5 = <ShadecnInputOTPSlot> {}
        slot6 = <ShadecnInputOTPSlot> {}
    }
}
