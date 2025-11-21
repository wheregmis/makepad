use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCarouselItem = <RoundedView> {
        width: 200.0, height: 200.0 // Default size
        show_bg: true
        draw_bg: {
            color: (COLOR_BG_SECONDARY)
            border_radius: (RADIUS_MD)
        }
        align: {x: 0.5, y: 0.5}
        <Label> { text: "1" }
    }

    pub ShadecnCarousel = {{ShadecnCarousel}} {
        width: Fill, height: Fit
        flow: Down
        spacing: (SPACE_4)
        
        // Viewport
        viewport = <ScrollXView> {
            width: Fill, height: Fit
            
            content = <View> {
                width: Fit, height: Fit
                flow: Right
                spacing: (SPACE_4)
                
                <ShadecnCarouselItem> {}
                <ShadecnCarouselItem> { <Label> { text: "2" } }
                <ShadecnCarouselItem> { <Label> { text: "3" } }
                <ShadecnCarouselItem> { <Label> { text: "4" } }
                <ShadecnCarouselItem> { <Label> { text: "5" } }
            }
        }
        
        // Controls
        controls = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {x: 0.5, y: 0.5}
            spacing: (SPACE_2)
            
            <Button> { text: "Prev" }
            <Button> { text: "Next" }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnCarousel {
    #[live] viewport: View,
    #[live] controls: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnCarousel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.viewport.handle_event(cx, event, scope);
        self.controls.handle_event(cx, event, scope);
        
        // Logic for scrolling would go here
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let _ = self.viewport.draw_all(cx, scope);
        let _ = self.controls.draw_all(cx, scope);
        cx.end_turtle();
        DrawStep::done()
    }
}

