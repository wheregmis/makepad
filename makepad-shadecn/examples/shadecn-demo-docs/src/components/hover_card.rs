use makepad_shadecn_core::*;
use makepad_shadecn_hover_card::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub HoverCardShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Hover Card"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "For sighted users to preview content available behind a link."
            }
        }
        ComponentSectionContent = {
            hover_card_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Hover to show card content."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnHoverCard> {
                        trigger: <View> {
                            <Label> { 
                                text: "Hover over me"
                                draw_text: { color: (COLOR_PRIMARY) }
                            }
                        }
                        content: <View> {
                            padding: (SPACE_4)
                            <Label> { text: "Hover card content" }
                        }
                    }
                }
            }
        }
    }
}
