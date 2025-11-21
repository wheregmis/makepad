use makepad_shadecn_core::*;
use makepad_shadecn_tooltip::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub TooltipShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Tooltip"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A popup that displays information related to an element when the element receives keyboard focus or the mouse hovers over it."
            }
        }
        ComponentSectionContent = {
            tooltip_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Hover to show tooltip."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnTooltip> {}
                }
            }
        }
    }
}
