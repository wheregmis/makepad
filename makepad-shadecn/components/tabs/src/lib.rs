use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnTabTrigger = <ButtonFlat> {
        width: Fit, height: Fit
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}
        
        draw_bg: {
            color: (COLOR_BG_SECONDARY)
        }
        
        draw_text: {
            color: (COLOR_FG_TERTIARY)
            text_style: { font_size: (FONT_SM) }
        }
        
        text: "Tab"
    }

    pub ShadecnTabsList = <RoundedView> {
        width: Fit, height: Fit
        flow: Right
        padding: (SPACE_1)
        spacing: (SPACE_1)
        
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_SECONDARY)
            border_radius: (RADIUS_MD)
        }
    }

    pub ShadecnTabsContent = <View> {
        width: Fill, height: Fit
        visible: true
    }

    pub ShadecnTabs = {{ShadecnTabs}} {
        width: Fill, height: Fit
        flow: Down
        spacing: (SPACE_4)
        
        list = <ShadecnTabsList> {
            <ShadecnTabTrigger> { text: "Account" }
            <ShadecnTabTrigger> { text: "Password" }
        }
        
        // Container for contents
        content = <View> {
            width: Fill, height: Fit
            
            <Label> { text: "Tab content goes here" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnTabs {
    #[live] list: View,
    #[live] content: View,
    
    #[deref] view: View,
}

impl Widget for ShadecnTabs {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.list.handle_event(cx, event, scope);
        self.content.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, Layout::flow_down());
        let _ = self.list.draw_all(cx, scope);
        let _ = self.content.draw_all(cx, scope);
        cx.end_turtle();
        DrawStep::done()
    }
}
