use makepad_shadecn_core::*;
use makepad_shadecn_toast::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub ToastShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Toast"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A succinct message that is displayed temporarily."
            }
        }
        ComponentSectionContent = {
            toast_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Toast notifications appear at the bottom-right."
                    }
                }
                ComponentSubsectionDemo = {
                    <Label> { 
                        text: "Toast component available"
                        draw_text: { color: (COLOR_FG_TERTIARY) }
                    }
                }
            }
        }
    }
}
