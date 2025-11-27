use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_tabs::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_core;
    use link::shadecn_core::*;
    
    link shadecn_tabs;
    use link::shadecn_tabs::*;

    pub TabsShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Tabs"
            }
            ComponentSectionDescription = {
                text: "A set of layered sections of content—known as tab panels—that are displayed one at a time."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Tabs component allows users to switch between different views.\n\n## Usage\n\nImport the tabs component:\n\n```rust\nuse makepad_shadecn_tabs::*;\n```"
                }
            }

            tabs_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Tabs"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard tabs component."
                    }
                }
                ComponentSubsectionDemo = {
                    tabs_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        
                        tabs_list = <ShadecnTabsList> {
                            <ShadecnTabTrigger> { text: "Account", animator: {active = {default: on}} }
                            <ShadecnTabTrigger> { text: "Password" }
                        }
                        
                        tabs_content = <ShadecnTabsContent> {
                            <Label> { text: "Make changes to your account here." }
                        }
                    }
                }
            }
        }
    }
}
