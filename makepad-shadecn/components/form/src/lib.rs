use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnFormItem = <View> {
        width: Fill, height: Fit,
        flow: Down,
        spacing: (SPACE_2)
    }

    pub ShadecnFormLabel = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnFormDescription = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_FG_MUTED)
        }
    }

    pub ShadecnFormMessage = <Label> {
        width: Fit, height: Fit,
        draw_text: {
            text_style: { font_size: (FONT_SM) }
            color: (COLOR_DESTRUCTIVE)
        }
    }
}
