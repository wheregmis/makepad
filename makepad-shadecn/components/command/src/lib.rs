use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCommand = {{ShadecnCommand}} {
        width: 400.0
        height: 300.0
        flow: Down
        
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
        }
        
        // Search input area
        <View> {
            width: Fill, height: Fit
            padding: (SPACE_3)
            
            <TextInput> {
                width: Fill, height: Fit
                text: "Type a command or search..."
                draw_bg: { color: (COLOR_BG_PRIMARY) }
                draw_text: { color: (COLOR_FG_PRIMARY) }
            }
        }
        
        // List area
        <PortalList> {
            width: Fill, height: Fill
            // Items would go here
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnCommand {
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnCommand {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
