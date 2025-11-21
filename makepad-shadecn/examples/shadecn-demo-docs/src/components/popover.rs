use makepad_shadecn_core::*;
use makepad_shadecn_popover::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub PopoverShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Popover"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays rich content in a portal, triggered by a button."
            }
        }
        ComponentSectionContent = {
            popover_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Click to toggle popover content."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnPopover> {}
                }
            }
        }
    }
}
