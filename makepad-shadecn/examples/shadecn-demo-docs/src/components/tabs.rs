use makepad_shadecn_core::*;
use makepad_shadecn_tabs::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub TabsShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Tabs"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A set of layered sections of content—known as tab panels—that are displayed one at a time."
            }
        }
        ComponentSectionContent = {
            tabs_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Switch between different content sections."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnTabs> {}
                }
            }
        }
    }
}
