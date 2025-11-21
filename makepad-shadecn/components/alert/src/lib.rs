use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnAlert = {{ShadecnAlert}} {
        width: Fill, height: Fit
        flow: Right
        padding: (SPACE_4)
        spacing: (SPACE_4)
        
        icon = <View> {
             width: Fit, height: Fit
             // Placeholder for icon
             <Label> { text: "i", draw_text: { color: (COLOR_FG_TERTIARY) } }
        }
        
        content = <View> {
            width: Fill, height: Fit
            flow: Down
            spacing: (SPACE_1)
            
            title = <Label> {
                draw_text: {
                    text_style: { font_size: (FONT_BASE) }
                    color: (COLOR_FG_PRIMARY)
                }
                text: "Heads up!"
            }
            
            description = <Label> {
                draw_text: {
                    text_style: { font_size: (FONT_SM) }
                    color: (COLOR_FG_PRIMARY)
                }
                text: "You can add components to your app using the cli."
            }
        }
    }
    
    pub ShadecnAlertDestructive = <ShadecnAlert> {
        content = {
            title = { draw_text: { color: (COLOR_ERROR) } }
            description = { draw_text: { color: (COLOR_ERROR) } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnAlert {
    #[live] icon: View,
    #[live] content: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnAlert {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
