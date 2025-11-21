use makepad_shadecn_core::*;
use makepad_shadecn_dialog::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub DialogShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Dialog"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A modal dialog that interrupts the user with important content and expects a response."
            }
        }
        ComponentSectionContent = {
            dialog_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple dialog with trigger button."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnDialog> {}
                }
            }
        }
    }
}
