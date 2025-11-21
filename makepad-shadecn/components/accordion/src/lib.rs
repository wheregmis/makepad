use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnAccordionItem = {{ShadecnAccordionItem}} {
        width: Fill, height: Fit
        flow: Down
        
        header = <View> {
            width: Fill, height: Fit
            padding: {top: (SPACE_4), bottom: (SPACE_4)}
            flow: Right
            align: {x: 0.0, y: 0.5}
            cursor: Hand
            
            label = <Label> {
                width: Fill
                draw_text: {
                    text_style: { font_size: (FONT_BASE) }
                    color: (COLOR_FG_PRIMARY)
                }
                text: "Is it accessible?"
            }
            
            icon = <Label> {
                text: "v" // Chevron down
                draw_text: { color: (COLOR_FG_TERTIARY) }
            }
        }
        
        content = <View> {
            width: Fill, height: Fit
            padding: {bottom: (SPACE_4)}
            
            label = <Label> {
                width: Fill
                draw_text: {
                    text_style: { font_size: (FONT_SM) }
                    color: (COLOR_FG_TERTIARY)
                    wrap: Word
                }
                text: "Yes. It adheres to the WAI-ARIA design pattern."
            }
        }
        
        animator: {
            open = {
                default: off
                off = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {content = {height: 0.0, visible: false}}
                }
                on = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {content = {height: Fit, visible: true}}
                }
            }
        }
    }
    
    pub ShadecnAccordion = <View> {
        width: Fill, height: Fit
        flow: Down
        
        <ShadecnAccordionItem> {}
        <ShadecnAccordionItem> {
            header = { label = { text: "Is it styled?" } }
            content = { label = { text: "Yes. It comes with default styles that matches the other components' aesthetic." } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnAccordionItem {
    #[live] header: View,
    #[live] content: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[animator] animator: Animator,
    
    #[rust] open: bool,
    
    #[deref] view: View,
}

impl Widget for ShadecnAccordionItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.header.handle_event(cx, event, scope);
        
        if matches!(event.hits(cx, self.header.area()), Hit::FingerDown(_)) {
            self.open = !self.open;
            if self.open {
                self.animator_play(cx, ids!(open.on));
            } else {
                self.animator_play(cx, ids!(open.off));
            }
        }
        
        if self.open || self.animator_in_state(cx, ids!(open.on)) {
            self.content.handle_event(cx, event, scope);
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        
        let _ = self.header.draw_all(cx, scope);
        let _ = self.content.draw_all(cx, scope);
        
        // Draw bottom border
        // let rect = cx.turtle().rect();
        // cx.draw_line(...) - replaced by View properties or DrawQuad if needed
        
        cx.end_turtle();
        
        DrawStep::done()
    }
}

