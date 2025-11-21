use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnDrawerContent = <RoundedView> {
        width: Fill
        height: 300.0
        flow: Down
        
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_size: 1.0
            border_radius: 0.0
        }
        
        padding: (SPACE_6)
        spacing: (SPACE_4)
        
        // Handle bar
        <View> {
            width: Fill, height: Fit
            align: {x: 0.5, y: 0.0}
            <RoundedView> {
                width: 40.0, height: 4.0
                show_bg: true
                draw_bg: {
                    color: (COLOR_BG_TERTIARY)
                    border_radius: 2.0
                }
            }
        }
    }

    pub ShadecnDrawer = {{ShadecnDrawer}} {
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
                text: "Open Drawer"
                draw_text: { color: (COLOR_PRIMARY_FOREGROUND) }
            }
        }
        
        modal: <Modal> {
            align: {x: 0.5, y: 1.0} // Align bottom
            
            bg_view: {
                draw_bg: {
                    color: (COLOR_DARK_TRANSPARENT_47)
                }
            }
            
            content: <ShadecnDrawerContent> {
                // Default content
                <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: (SPACE_2)
                    
                    <Label> {
                        draw_text: {
                            text_style: { font_size: (FONT_LG) }
                            color: (COLOR_FG_PRIMARY)
                        }
                        text: "Drawer Title"
                    }
                    
                    <Label> {
                        draw_text: {
                            text_style: { font_size: (FONT_SM) }
                            color: (COLOR_FG_TERTIARY)
                        }
                        text: "Drawer Description"
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnDrawer {
    #[live] trigger: View,
    #[live] modal: Modal,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnDrawer {
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
