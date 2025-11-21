use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnCard = <RoundedView> {
        width: Fit,
        height: Fit,
        flow: Down,

        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_PRIMARY),
            border_color: (COLOR_BORDER_PRIMARY),
            border_size: 1.0,
            border_radius: (RADIUS_XL),
        }
    }

    pub ShadecnCardHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_1)},
        spacing: (SPACE_1),
    }

    pub ShadecnCardTitle = <Label> {
        width: Fit,
        height: Fit,
        draw_text: {
            text_style: {
                font_size: (FONT_2XL),
            }
            color: (COLOR_FG_PRIMARY)
        }
    }

    pub ShadecnCardDescription = <Label> {
        width: Fit,
        height: Fit,
        draw_text: {
            text_style: {
                font_size: (FONT_SM),
            }
            color: (COLOR_FG_TERTIARY)
        }
    }

    pub ShadecnCardContent = <View> {
        width: Fill,
        height: Fit,
        padding: {left: (SPACE_6), top: 0.0, right: (SPACE_6), bottom: (SPACE_6)},
    }

    pub ShadecnCardFooter = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        align: {x: 0.0, y: 0.5},
        padding: {left: (SPACE_6), top: 0.0, right: (SPACE_6), bottom: (SPACE_6)},
    }
}
