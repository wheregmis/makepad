use makepad_shadecn_core::*;
use makepad_shadecn_context_menu::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub ContextMenuShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Context Menu"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
            }
        }
        ComponentSectionContent = {
            context_menu_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Right-click to open context menu."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnContextMenu> {}
                }
            }
        }
    }
}
