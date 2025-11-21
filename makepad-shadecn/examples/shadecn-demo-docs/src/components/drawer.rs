use makepad_shadecn_core::*;
use makepad_shadecn_drawer::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub DrawerShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Drawer"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A panel that slides in from the bottom of the screen."
            }
        }
        ComponentSectionContent = {
            drawer_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A bottom drawer component."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnDrawer> {}
                }
            }
        }
    }
}
