use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnToggleGroup = <View> {
        width: Fit, height: Fit,
        flow: Right,
        spacing: 1.0
    }

    pub ShadecnToggleGroupItem = <RadioButton> {
        width: Fit, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        align: {x: 0.5, y: 0.5}
        
        draw_radio: {
            instance hover: 0.0
            instance focus: 0.0
            instance selected: 0.0
            instance down: 0.0
            
            color: (COLOR_BG_PRIMARY)
            color_selected: (COLOR_ACCENT)
            color_hover: (COLOR_ACCENT_HOVER)
            
            border_radius: (RADIUS_SM)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    0.0,
                    0.0,
                    self.rect_size.x,
                    self.rect_size.y,
                    self.border_radius
                );
                
                let bg_color = self.color;
                if self.selected > 0.5 {
                    bg_color = self.color_selected;
                } else if self.hover > 0.5 {
                    bg_color = self.color_hover;
                }
                
                sdf.fill(bg_color);
                return sdf.result;
            }
        }
        
        draw_text: {
            text_style: { font_size: (FONT_BASE) }
            color: (COLOR_FG_PRIMARY)
            color_selected: (COLOR_FG_ACCENT)
            
            fn get_color(self) -> vec4 {
                if self.selected > 0.5 {
                    return self.color_selected;
                }
                return self.color;
            }
        }
        
        draw_icon: {
            color: (COLOR_FG_PRIMARY)
            color_selected: (COLOR_FG_ACCENT)
            
            fn get_color(self) -> vec4 {
                if self.selected > 0.5 {
                    return self.color_selected;
                }
                return self.color;
            }
        }
        
        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_radio: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_radio: {hover: 1.0}
                    }
                }
                down = {
                    from: {all: Snap}
                    apply: {
                        draw_radio: {down: 1.0}
                    }
                }
            }
            selected = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_radio: {selected: 0.0}
                        draw_text: {selected: 0.0}
                        draw_icon: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_radio: {selected: 1.0}
                        draw_text: {selected: 1.0}
                        draw_icon: {selected: 1.0}
                    }
                }
            }
        }
    }
}
