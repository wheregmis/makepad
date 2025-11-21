use makepad_shadecn_core::theme::*;
use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnSkeleton = <RoundedView> {
        width: Fill,
        height: Fill,
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_SECONDARY),
            border_radius: (RADIUS_MD),
        }

        animator: {
            pulse = {
                default: on,
                on = {
                    from: {all: Forward {duration: 1.5, loop: 1}}
                    key_0 = {
                        time: 0.0,
                        apply: {
                            draw_bg: {color: (COLOR_BG_SECONDARY)}
                        }
                    }
                    key_1 = {
                        time: 0.5,
                        apply: {
                            draw_bg: {color: (COLOR_BG_TERTIARY)}
                        }
                    }
                    key_2 = {
                        time: 1.0,
                        apply: {
                            draw_bg: {color: (COLOR_BG_SECONDARY)}
                        }
                    }
                }
            }
        }
    }
}
