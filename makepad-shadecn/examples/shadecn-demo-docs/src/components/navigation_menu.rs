use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_navigation_menu::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_navigation_menu;
    use link::shadecn_navigation_menu::*;

    pub NavigationMenuShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Navigation Menu"
            }
            ComponentSectionDescription = {
                text: "A collection of links for navigating websites."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Navigation Menu component is used for site navigation.\n\n## Usage\n\nImport the navigation menu component:\n\n```rust\nuse makepad_shadecn_navigation_menu::*;\n```"
                }
            }

            navigation_menu_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Navigation Menu"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard navigation menu."
                    }
                }
                ComponentSubsectionDemo = {
                    navigation_menu_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: (SPACE_3),
                        
                        <ShadecnNavigationMenu> {}
                    }
                }
            }
        }
    }
}
