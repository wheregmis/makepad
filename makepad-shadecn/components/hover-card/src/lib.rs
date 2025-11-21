use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnHoverCard = {{ShadecnHoverCard}} {
        width: Fit,
        height: Fit,
        
        trigger: <View> {
            width: Fit, height: Fit
            cursor: Hand
        }
        
        content: <RoundedView> {
            width: Fit, height: Fit
            flow: Down
            padding: {left: (SPACE_4), top: (SPACE_3), right: (SPACE_4), bottom: (SPACE_3)}
            
            show_bg: true
            draw_bg: {
                color: (COLOR_BG_POPOVER)
                border_color: (COLOR_BORDER_PRIMARY)
                border_size: 1.0
                border_radius: (RADIUS_MD)
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnHoverCard {
    #[live] trigger: View,
    #[live] content: View,
    
    #[redraw] #[rust(DrawList2d::new(cx))] draw_list: DrawList2d,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
    
    #[rust] hover_timer: Timer,
    #[rust] opened: bool,
}

impl Widget for ShadecnHoverCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.trigger.handle_event(cx, event, scope);
        
        match event.hits(cx, self.trigger.area()) {
            Hit::FingerHoverIn(_) => {
                self.opened = true;
                self.draw_list.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                // In a real implementation we might want a delay
                // For now, close immediately but we need to check if we moved into content
                 self.opened = false;
                 self.draw_list.redraw(cx);
            }
            _ => {}
        }
        
        // Also handle content hover to keep it open? 
        // This is tricky with current hit logic if content is overlay.
        // Simplified: Hover card just shows when hovering trigger.
        
        if self.opened {
             self.content.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let _ = self.trigger.draw_all(cx, scope);
        
        // Get trigger rect before ending turtle
        let trigger_rect = if self.opened {
            let rect = self.trigger.area().rect(cx);
            // Validate rect - if invalid, don't show overlay
            if rect.size.x > 0.0 && rect.size.y > 0.0 && rect.pos.x.is_finite() && rect.pos.y.is_finite() {
                Some(rect)
            } else {
                None
            }
        } else {
            None
        };
        
        cx.end_turtle();
        
        if let Some(rect) = trigger_rect {
            self.draw_list.begin_overlay_reuse(cx);
            
            let pos = rect.pos + dvec2(0.0, rect.size.y + 5.0);
            
            // Ensure position is valid (finite and non-negative)
            if pos.x.is_finite() && pos.y.is_finite() && pos.x >= 0.0 && pos.y >= 0.0 {
                let pass_size = cx.current_pass_size();
                cx.begin_root_turtle(pass_size, Layout::flow_down());
                
                // Use margin in walk to position the content
                let positioned_walk = Walk {
                    width: Size::fit(),
                    height: Size::fit(),
                    margin: Margin { left: pos.x, top: pos.y, right: 0.0, bottom: 0.0 },
                    abs_pos: None,
                    metrics: Metrics::default(),
                };
                
                let _ = self.content.draw_walk(cx, scope, positioned_walk);
                
                cx.end_pass_sized_turtle();
            }
            self.draw_list.end(cx);
        }
        
        DrawStep::done()
    }
}

