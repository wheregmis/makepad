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

                        <ShadecnLabel> { text: "Line 1" }
                        <ShadecnLabel> { text: "Line 2" }
                        <ShadecnLabel> { text: "Line 3" }
                        <ShadecnLabel> { text: "Line 4" }
                        <ShadecnLabel> { text: "Line 5" }
                        <ShadecnLabel> { text: "Line 6" }
                        <ShadecnLabel> { text: "Line 7" }
                        <ShadecnLabel> { text: "Line 8" }
                        <ShadecnLabel> { text: "Line 9" }
                        <ShadecnLabel> { text: "Line 10" }
                        <ShadecnLabel> { text: "Line 11" }
                        <ShadecnLabel> { text: "Line 12" }
                        <ShadecnLabel> { text: "Line 13" }
                        <ShadecnLabel> { text: "Line 14" }
                        <ShadecnLabel> { text: "Line 15" }
                        <ShadecnLabel> { text: "Line 16" }
                        <ShadecnLabel> { text: "Line 17" }
                        <ShadecnLabel> { text: "Line 18" }
                        <ShadecnLabel> { text: "Line 19" }
                        <ShadecnLabel> { text: "Line 20" }
                    }
                }
            }
        }
    }
}
