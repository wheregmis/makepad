use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_menubar::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    link shadecn_menubar;
    use link::shadecn_menubar::*;
    
    // Local doc wrapper mirroring ShadecnMenubar layout to avoid missing live links in hot reload
    pub DocMenubar = <RoundedView> {
        width: Fit, height: Fit,
        flow: Right,
        padding: (SPACE_1),
        spacing: (SPACE_1),
        show_bg: true,
        draw_bg: {
            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_size: 1.0
            border_radius: (RADIUS_MD)
        }

        file = <Button> { text: "File" }
        edit = <Button> { text: "Edit" }
        view = <Button> { text: "View" }
        profile = <Button> { text: "Profile" }
    }

    pub MenubarShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Menubar"
            }
            ComponentSectionDescription = {
                text: "A visually persistent menu common in desktop applications."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Menubar component provides a top-level menu for your application.\n\n## Usage\n\nImport the menubar component:\n\n```rust\nuse makepad_shadecn_menubar::*;\n```"
                }
            }

            menubar_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Menubar"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard menubar with items."
                    }
                }
                ComponentSubsectionDemo = {
                    menubar_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: (SPACE_3),
                        
                        <DocMenubar> {}
                    }
                }
            }
        }
    }
}
