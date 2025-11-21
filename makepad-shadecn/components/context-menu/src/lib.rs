use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnContextMenu = {{ShadecnContextMenu}} {
        width: Fit, height: Fit
        
        trigger: <RoundedView> {
            width: 200.0, height: 100.0
            show_bg: true
            draw_bg: {
                color: (COLOR_BG_SECONDARY)
                border_radius: (RADIUS_MD)
                border_color: (COLOR_BORDER_PRIMARY)
                border_size: 1.0
            }
            align: {x: 0.5, y: 0.5}
            <Label> { text: "Right click here" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnContextMenu {
    #[live] trigger: View,
    
    #[deref] view: View,
}

impl Widget for ShadecnContextMenu {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.trigger.handle_event(cx, event, scope);
        // Context menu logic would require right-click detection
        // For now, simplified
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.trigger.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}
