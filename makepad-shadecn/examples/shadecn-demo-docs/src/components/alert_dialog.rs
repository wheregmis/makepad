use makepad_shadecn_core::*;
use makepad_shadecn_alert_dialog::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub AlertDialogShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Alert Dialog"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A modal dialog that interrupts the user with important content and expects a response."
            }
        }
        ComponentSectionContent = {
            alert_dialog_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "An alert dialog for critical actions."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnAlertDialog> {}
                }
            }
        }
    }
}
