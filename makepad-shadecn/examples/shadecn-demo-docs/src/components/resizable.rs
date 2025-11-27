use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_resizable::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_resizable;
    use link::shadecn_resizable::*;

    pub ResizableShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Resizable"
            }
            ComponentSectionDescription = {
                text: "Accessible resizable panel groups and layouts."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Resizable component allows users to resize panels.\n\n## Usage\n\nImport the resizable component:\n\n```rust\nuse makepad_shadecn_resizable::*;\n```"
                }
            }

            resizable_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Resizable"
                    }
                    ComponentSubsectionDescription = {
                        text: "A resizable layout."
                    }
                }
                ComponentSubsectionDemo = {
                    resizable_demo = <View> {
                        width: Fill,
                        height: 200.0,
                        flow: Right,
                        spacing: (SPACE_3),
                        
                        <ShadecnResizable> {
                            a: <View> {
                                width: Fill, height: Fill,
                                show_bg: true,
                                draw_bg: { color: (COLOR_BG_SECONDARY) }
                                align: {x: 0.5, y: 0.5}
                                <Label> { text: "One" }
                            }
                            b: <View> {
                                width: Fill, height: Fill,
                                show_bg: true,
                                draw_bg: { color: (COLOR_BG_SECONDARY) }
                                align: {x: 0.5, y: 0.5}
                                <Label> { text: "Two" }
                            }
                        }
                    }
                }
            }
        }
    }
}
