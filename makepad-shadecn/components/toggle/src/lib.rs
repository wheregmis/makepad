use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnToggle = {{ShadecnToggle}} {
        width: Fit, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        align: {x: 0.5, y: 0.5}
        spacing: (SPACE_2)
        
        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance selected: 0.0
            instance down: 0.0
            
            color: (COLOR_BG_PRIMARY) // Transparent/Base
            color_selected: (COLOR_ACCENT)
            color_hover: (COLOR_ACCENT_HOVER)
            
            border_radius: (RADIUS_MD)
            
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
                        draw_bg: {hover: 0.0}
                    }
                }
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {hover: 1.0}
                    }
                }
                down = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {down: 1.0}
                    }
                }
            }
            selected = {
                default: off,
                off = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {selected: 0.0}
                        draw_text: {selected: 0.0}
                        draw_icon: {selected: 0.0}
                    }
                }
                on = {
                    from: {all: Snap}
                    apply: {
                        draw_bg: {selected: 1.0}
                        draw_text: {selected: 1.0}
                        draw_icon: {selected: 1.0}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnToggle {
    #[animator] animator: Animator,
    #[redraw] #[live] draw_bg: DrawQuad,
    #[live] draw_text: DrawText,
    #[live] draw_icon: DrawIcon,
    
    #[live] icon_walk: Walk,
    #[live] text: String,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[live] selected: bool,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum ToggleAction {
    None,
    Change(bool),
}

impl Widget for ShadecnToggle {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(_fe) => {
                self.animator_play(cx, ids!(hover.down));
            }
            Hit::FingerUp(fe) => {
                if fe.is_over {
                    self.selected = !self.selected;
                    // Trigger redraw and animation
                    if self.selected {
                        self.animator_play(cx, ids!(selected.on));
                    } else {
                        self.animator_play(cx, ids!(selected.off));
                    }
                    cx.widget_action(self.widget_uid(), &scope.path, ToggleAction::Change(self.selected));
                    self.draw_bg.redraw(cx);
                }
                if fe.is_over {
                     self.animator_play(cx, ids!(hover.on));
                } else {
                     self.animator_play(cx, ids!(hover.off));
                }
            }
            Hit::FingerHoverIn(_) => {
                self.animator_play(cx, ids!(hover.on));
            }
            Hit::FingerHoverOut(_) => {
                self.animator_play(cx, ids!(hover.off));
            }
            _ => ()
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        if self.draw_icon.icon_id != 0 {
             self.draw_icon.draw_walk(cx, self.icon_walk);
        }
        if !self.text.is_empty() {
            self.draw_text.draw_walk(cx, Walk::fit(), Align::default(), &self.text);
        }
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
