use makepad_shadecn_core::*;
use makepad_shadecn_scroll_area::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    // ScrollArea Component Showcase
    pub ScrollAreaShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Scroll Area"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Augments native scroll functionality for custom, cross-browser styling."
            }
        }
        ComponentSectionContent = {
            scroll_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A scrollable container with custom scrollbars."
                    }
                }
                ComponentSubsectionDemo = {
                    scroll_demo = <ShadecnScrollArea> {
                        width: Fill,
                        height: 200,
                        flow: Down,
                        spacing: (SPACE_2),

                        <Label> { text: "Line 1" }
                        <Label> { text: "Line 2" }
                        <Label> { text: "Line 3" }
                        <Label> { text: "Line 4" }
                        <Label> { text: "Line 5" }
                        <Label> { text: "Line 6" }
                        <Label> { text: "Line 7" }
                        <Label> { text: "Line 8" }
                        <Label> { text: "Line 9" }
                        <Label> { text: "Line 10" }
                        <Label> { text: "Line 11" }
                        <Label> { text: "Line 12" }
                        <Label> { text: "Line 13" }
                        <Label> { text: "Line 14" }
                        <Label> { text: "Line 15" }
                        <Label> { text: "Line 16" }
                        <Label> { text: "Line 17" }
                        <Label> { text: "Line 18" }
                        <Label> { text: "Line 19" }
                        <Label> { text: "Line 20" }
                    }
                }
            }
        }
    }
}
