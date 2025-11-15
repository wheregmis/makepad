use makepad_widgets::*;

live_design! {
    link widgets;
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    pub ShadecnLabelBase = <Label> {
        draw_text: {
            text_style: {
                font_size: (FONT_BASE),
            }
            color: (COLOR_FG_PRIMARY),
        }
    }

    // Variants for different text styles
    pub ShadecnLabel = <ShadecnLabelBase> {}

    pub ShadecnLabelSecondary = <ShadecnLabelBase> {
        draw_text: {
            color: (COLOR_FG_SECONDARY),
        }
    }

    pub ShadecnLabelTertiary = <ShadecnLabelBase> {
        draw_text: {
            color: (COLOR_FG_TERTIARY),
        }
    }

    pub ShadecnLabelDisabled = <ShadecnLabelBase> {
        draw_text: {
            color: (COLOR_FG_DISABLED),
        }
    }

    pub ShadecnLabelSm = <ShadecnLabelBase> {
        draw_text: {
            text_style: {
                font_size: (FONT_SM),
            }
        }
    }

    pub ShadecnLabelLg = <ShadecnLabelBase> {
        draw_text: {
            text_style: {
                font_size: (FONT_LG),
            }
        }
    }
}
