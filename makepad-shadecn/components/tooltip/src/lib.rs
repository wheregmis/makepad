use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnTooltipContent = <RoundedView> {
        width: Fit
        height: Fit
        padding: {left: (SPACE_2), right: (SPACE_2), top: (SPACE_1), bottom: (SPACE_1)}
        
        show_bg: true
        draw_bg: {
            color: (COLOR_FG_PRIMARY) // Dark background for tooltip usually
            border_radius: (RADIUS_SM)
        }
        
        label = <Label> {
            draw_text: {
                text_style: { font_size: (FONT_XS) }
                color: (COLOR_BG_PRIMARY) // Light text
            }
            text: "Tooltip text"
        }
    }

    pub ShadecnTooltip = {{ShadecnTooltip}} {
        width: Fit, height: Fit
        
        trigger: <RoundedView> {
            width: Fit, height: Fit
            padding: {left: (SPACE_3), top: 6, right: (SPACE_3), bottom: 6}
            show_bg: true
            cursor: Hand
            draw_bg: {
                color: (COLOR_BG_SECONDARY)
                border_radius: (RADIUS_MD)
            }
            <Label> {
                text: "Hover"
                draw_text: { color: (COLOR_FG_PRIMARY) }
            }
        }
        
        content: <ShadecnTooltipContent> {}
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnTooltip {
    #[live] trigger: View,
    #[live] content: View,
    
    #[redraw] #[rust(DrawList2d::new(cx))] draw_list: DrawList2d,
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
    
    #[rust] visible: bool,
}

impl Widget for ShadecnTooltip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.trigger.handle_event(cx, event, scope);
        
        match event.hits(cx, self.trigger.area()) {
            Hit::FingerHoverIn(_) => {
                self.visible = true;
                self.draw_list.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.visible = false;
                self.draw_list.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let _ = self.trigger.draw_all(cx, scope);
        
        // Get trigger rect before ending turtle
        let trigger_rect = if self.visible {
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
            
            // Position above trigger - ensure we don't go negative
            let tooltip_height = 30.0; // Approximate tooltip height
            let pos_y = (rect.pos.y - tooltip_height).max(0.0);
            let pos = dvec2(rect.pos.x, pos_y);
            
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
