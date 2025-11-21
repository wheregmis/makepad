use makepad_shadecn_core::*;
use makepad_shadecn_table::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub TableShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Table"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A responsive table component."
            }
        }
        ComponentSectionContent = {
            table_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Display data in a structured table format."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnTable> {}
                }
            }
        }
    }
}
