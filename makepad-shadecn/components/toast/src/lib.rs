use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnToast = {{ShadecnToast}} {
        width: 300.0, height: Fit
        
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_size: 1.0
            border_radius: (RADIUS_MD)
            
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.0,
                    self.rect_size.y - self.border_size * 2.0,
                    self.border_radius
                );
                sdf.fill(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }
        
        flow: Overlay
        align: {x: 1.0, y: 1.0} // Bottom right
        padding: (SPACE_4)
        
        content: <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: (SPACE_1)
            
            <Label> {
                draw_text: {
                    text_style: { font_size: (FONT_BASE), font_weight: 600.0 }
                    color: (COLOR_FG_PRIMARY)
                }
                text: "Scheduled: Catch up"
            }
            <Label> {
                draw_text: {
                    text_style: { font_size: (FONT_SM) }
                    color: (COLOR_FG_TERTIARY)
                }
                text: "Friday, February 10, 2023 at 5:57 PM"
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnToast {
    #[live] content: View,
    #[live] draw_bg: DrawQuad,
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[redraw] #[rust(DrawList2d::new(cx))] draw_list: DrawList2d,
    
    #[deref] view: View,
    
    #[rust] visible: bool,
}

impl Widget for ShadecnToast {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.visible {
            self.content.handle_event(cx, event, scope);
             // Click to dismiss
            if matches!(event.hits(cx, self.draw_bg.area()), Hit::FingerDown(_)) {
                self.visible = false;
                self.draw_list.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        if !self.visible {
            return DrawStep::done();
        }

        self.draw_list.begin_overlay_reuse(cx);
        
        // Position at bottom right of screen
        let size = cx.current_pass_size();
        let toast_width = 300.0;
        let toast_height = 80.0; // Approx
        let margin = 20.0;
        
        let pos = dvec2(size.x - toast_width - margin, size.y - toast_height - margin);
        
        cx.begin_turtle(Walk::fit().with_abs_pos(pos), self.layout);
        self.draw_bg.begin(cx, Walk::new(Size::Fixed(toast_width), Size::fit()), self.layout);
        
        let _ = self.content.draw_all(cx, scope);
        
        self.draw_bg.end(cx);
        cx.end_turtle();
        
        self.draw_list.end(cx);
        
        DrawStep::done()
    }
}

impl ShadecnToast {
    pub fn show(&mut self, cx: &mut Cx) {
        self.visible = true;
        self.draw_list.redraw(cx);
    }
}

