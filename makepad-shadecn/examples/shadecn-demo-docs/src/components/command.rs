use makepad_shadecn_core::*;
use makepad_shadecn_command::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub CommandShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Command"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Fast, composable, unstyled command menu for React."
            }
        }
        ComponentSectionContent = {
            command_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Search and execute commands."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnCommand> {}
                }
            }
        }
    }
}
