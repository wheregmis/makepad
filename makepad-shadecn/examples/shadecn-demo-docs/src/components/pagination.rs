use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_pagination::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_pagination;
    use link::shadecn_pagination::*;

    pub PaginationShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Pagination"
            }
            ComponentSectionDescription = {
                text: "Pagination with page navigation, next and previous links."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Pagination component allows users to navigate through pages of content.\n\n## Usage\n\nImport the pagination component:\n\n```rust\nuse makepad_shadecn_pagination::*;\n```"
                }
            }

            pagination_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Pagination"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard pagination component."
                    }
                }
                ComponentSubsectionDemo = {
                    pagination_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        
                        <ShadecnPagination> {
                            width: Fill,
                            height: Fit,
                        }
                    }
                }
            }
        }
    }
}
