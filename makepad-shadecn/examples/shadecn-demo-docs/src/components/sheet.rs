use makepad_shadecn_core::*;
use makepad_shadecn_sheet::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub SheetShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Sheet"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Extends the Dialog component to display content that complements the main content of the screen."
            }
        }
        ComponentSectionContent = {
            sheet_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A side sheet that slides in from the right."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnSheet> {}
                }
            }
        }
    }
}
