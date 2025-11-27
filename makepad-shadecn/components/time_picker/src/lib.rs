use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;
    use link::shadecn_input::*;

    pub ShadecnTimePickerInput = <ShadecnInput> {
        width: 60.0,
        height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_LG) }
        }
        align: {x: 0.5, y: 0.5}
    }

    pub ShadecnTimePicker = <View> {
        width: Fit, height: Fit,
        flow: Right,
        align: {y: 0.5},
        spacing: (SPACE_2),

        hour_input = <ShadecnTimePickerInput> { text: "12" }

        <Label> {
            text: ":",
            draw_text: {
                text_style: { font_size: (FONT_LG) }
                color: (COLOR_FG_PRIMARY)
            }
        }

        minute_input = <ShadecnTimePickerInput> { text: "00" }

        <Label> {
            text: ":",
            draw_text: {
                text_style: { font_size: (FONT_LG) }
                color: (COLOR_FG_PRIMARY)
            }
        }

        second_input = <ShadecnTimePickerInput> { text: "00" }
    }
}
