use makepad_shadecn_core::*;
use makepad_shadecn_accordion::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub AccordionShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Accordion"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A vertically stacked set of interactive headings that each reveal a section of content."
            }
        }
        ComponentSectionContent = {
            accordion_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Click to expand/collapse sections."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnAccordion> {}
                }
            }
        }
    }
}
