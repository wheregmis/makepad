use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::shadecn_button::*;

    pub ShadecnDatePicker = <ShadecnButtonOutline> {
        width: 240.0,
        text: "Pick a date",
        align: {x: 0.0, y: 0.5}

        // Placeholder for icon
        // draw_icon: {
        //     svg_file: dep("crate://self/resources/calendar.svg")
        // }
        // icon_walk: {width: 16.0, height: 16.0, margin: {right: 8.0}}
    }
}
