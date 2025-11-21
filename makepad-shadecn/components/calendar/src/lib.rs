use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCalendarDay = <View> {
        width: 40.0, height: 40.0
        show_bg: true
        cursor: Hand
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
        }
        align: {x: 0.5, y: 0.5}
        <Label> {
             draw_text: { color: (COLOR_FG_PRIMARY) }
             text: "1"
        }
    }

    pub ShadecnCalendar = {{ShadecnCalendar}} {
        width: Fit, height: Fit
        flow: Down
        spacing: (SPACE_2)
        padding: (SPACE_3)
        
        header = <View> {
            width: Fill, height: Fit
            flow: Right
            align: {x: 0.5, y: 0.5}
            padding: {bottom: (SPACE_4)}
            spacing: (SPACE_2)
            
            <Label> { text: "January 2024" }
        }
        
        // Week days
        <View> {
            width: Fit, height: Fit
            flow: Right
            <Label> { width: 40.0, align: {x: 0.5}, text: "Su", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "Mo", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "Tu", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "We", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "Th", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "Fr", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
            <Label> { width: 40.0, align: {x: 0.5}, text: "Sa", draw_text: { color: (COLOR_FG_TERTIARY), text_style: { font_size: (FONT_XS) } } }
        }
        
        // Days Grid - Simplified for demo
        days = <View> {
            width: 280.0, height: Fit // 7 * 40
            flow: RightWrap
            
            <ShadecnCalendarDay> { <Label> { text: "1" } }
            <ShadecnCalendarDay> { <Label> { text: "2" } }
            <ShadecnCalendarDay> { <Label> { text: "3" } }
            <ShadecnCalendarDay> { <Label> { text: "4" } }
            <ShadecnCalendarDay> { <Label> { text: "5" } }
            <ShadecnCalendarDay> { <Label> { text: "6" } }
            <ShadecnCalendarDay> { <Label> { text: "7" } }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnCalendar {
    #[live] header: View,
    #[live] days: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnCalendar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
