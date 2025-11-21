use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnDialogContent = <RoundedView> {
        width: 400.0
        height: Fit
        flow: Down
        
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_size: 1.0
            border_radius: (RADIUS_LG)
        }
        
        padding: (SPACE_6)
        spacing: (SPACE_4)
    }

    pub ShadecnDialog = {{ShadecnDialog}} {
        width: Fit, height: Fit
        
        trigger: <RoundedView> {
            width: Fit, height: Fit
            padding: {left: (SPACE_3), top: 6, right: (SPACE_3), bottom: 6}
            show_bg: true
            cursor: Hand
            draw_bg: {
                color: (COLOR_PRIMARY)
                border_radius: (RADIUS_MD)
            }
            <Label> {
                text: "Open Dialog"
                draw_text: { color: (COLOR_PRIMARY_FOREGROUND) }
            }
        }
        
        modal: <Modal> {
            bg_view: {
                draw_bg: {
                    color: (COLOR_DARK_TRANSPARENT_47)
                }
            }
            
            content: <ShadecnDialogContent> {
                // Default content example
                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: (SPACE_2)
                    
                    <Label> {
                        draw_text: {
                            text_style: { font_size: (FONT_LG) }
                            color: (COLOR_FG_PRIMARY)
                        }
                        text: "Dialog Title"
                    }
                    
                    <Label> {
                        draw_text: {
                            text_style: { font_size: (FONT_SM) }
                            color: (COLOR_FG_TERTIARY)
                        }
                        text: "Dialog Description"
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnDialog {
    #[live] trigger: View,
    #[live] modal: Modal,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnDialog {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.trigger.handle_event(cx, event, scope);
        
        if matches!(event.hits(cx, self.trigger.area()), Hit::FingerDown(_)) {
            self.modal.open(cx);
        }
        
        self.modal.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let _ = self.trigger.draw_all(cx, scope);
        cx.end_turtle();
        
        let _ = self.modal.draw_all(cx, scope);
        
        DrawStep::done()
    }
}
