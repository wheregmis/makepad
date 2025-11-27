use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnProgress = <RoundedView> {
        width: Fill,
        height: (SPACE_4),
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_SECONDARY),
            border_radius: (RADIUS_LG),
        }

        indicator = <RoundedView> {
            width: 0.0, // Set this programmatically or via DSL
            height: Fill,
            show_bg: true,
            draw_bg: {
                color: (COLOR_PRIMARY),
                border_radius: (RADIUS_LG),
            }
        }
    }
}
