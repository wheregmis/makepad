use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCollapsible = {{ShadecnCollapsible}} {
        width: Fill, height: Fit
        flow: Down
        
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {x: 0.0, y: 0.5}
            padding: (SPACE_2)
            
            label = <Label> {
                width: Fill
                text: "Can I use this in my project?"
            }
            
            toggle = <ButtonFlat> {
                width: Fit, height: Fit
                text: "+" 
                draw_bg: { color: (COLOR_WHITE_TRANSPARENT_00) } // Transparent
                draw_text: { color: (COLOR_FG_PRIMARY) }
            }
        }
        
        content = <View> {
            width: Fill, height: Fit
            padding: (SPACE_2)
            <Label> {
                text: "Yes. Free to use for personal and commercial projects. No attribution required."
                draw_text: {
                    wrap: Word
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnCollapsible {
    #[live] header: View,
    #[live] content: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[rust] open: bool,

    #[deref] view: View,
}

impl Widget for ShadecnCollapsible {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.header.handle_event(cx, event, scope);
        
        if matches!(event.hits(cx, self.header.area()), Hit::FingerDown(_)) {
            self.open = !self.open;
            self.redraw(cx);
        }
        
        if self.open {
            self.content.handle_event(cx, event, scope);
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        
        let _ = self.header.draw_all(cx, scope);
        
        if self.open {
            let _ = self.content.draw_all(cx, scope);
        }
        
        cx.end_turtle();
        
        DrawStep::done()
    }
}
