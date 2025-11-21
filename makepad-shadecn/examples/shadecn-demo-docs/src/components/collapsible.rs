use makepad_shadecn_core::*;
use makepad_shadecn_collapsible::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub CollapsibleShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Collapsible"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "An interactive component which expands/collapses a panel."
            }
        }
        ComponentSectionContent = {
            collapsible_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Click to toggle content visibility."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnCollapsible> {}
                }
            }
        }
    }
}
