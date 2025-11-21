use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnTableHeader = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: (SPACE_2)
        
        // Children should be Labels
    }

    pub ShadecnTableRow = <View> {
        width: Fill, height: Fit
        flow: Right
        padding: (SPACE_2)
        
        show_bg: true
        draw_bg: {
             color: (COLOR_BG_PRIMARY)
        }
    }

    pub ShadecnTableCell = <Label> {
        width: Fill, height: Fit
        padding: {top: (SPACE_2), bottom: (SPACE_2)}
        draw_text: {
            color: (COLOR_FG_PRIMARY)
            text_style: { font_size: (FONT_SM) }
        }
    }
    
    pub ShadecnTableHead = <Label> {
        width: Fill, height: Fit
        padding: {top: (SPACE_2), bottom: (SPACE_2)}
        draw_text: {
            color: (COLOR_FG_TERTIARY)
            text_style: { font_size: (FONT_SM) }
        }
    }

    pub ShadecnTable = {{ShadecnTable}} {
        width: Fill, height: Fit
        flow: Down
        
        // Header row
        header = <ShadecnTableHeader> {
            <ShadecnTableHead> { text: "Invoice" }
            <ShadecnTableHead> { text: "Status" }
            <ShadecnTableHead> { text: "Method" }
            <ShadecnTableHead> { text: "Amount", align: {x: 1.0} }
        }
        
        // Body
        body = <View> {
            width: Fill, height: Fit
            flow: Down
            
            <ShadecnTableRow> {
                <ShadecnTableCell> { text: "INV001" }
                <ShadecnTableCell> { text: "Paid" }
                <ShadecnTableCell> { text: "Credit Card" }
                <ShadecnTableCell> { text: "$250.00", align: {x: 1.0} }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ShadecnTable {
    #[live] header: View,
    #[live] body: View,
    
    #[layout] layout: Layout,
    #[walk] walk: Walk,
    
    #[deref] view: View,
}

impl Widget for ShadecnTable {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.header.handle_event(cx, event, scope);
        self.body.handle_event(cx, event, scope);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let _ = self.header.draw_all(cx, scope);
        let _ = self.body.draw_all(cx, scope);
        cx.end_turtle();
        DrawStep::done()
    }
}
