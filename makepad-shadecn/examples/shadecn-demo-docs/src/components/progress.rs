use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_progress::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    pub ProgressShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Progress"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Progress is used to show the progress of a task.\n\n## Usage\n\nImport from the `makepad-shadecn-progress` crate:\n\n```rust\nuse makepad_shadecn_progress::*;\n```"
                }
            }

            progress_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Examples"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Progress bar with different values."
                    }
                }
                ComponentSubsectionDemo = {
                    progress_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_4),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},
                        
                        <ShadecnProgress> {
                            indicator = { width: 150.0 }
                        }
                        
                        <ShadecnProgress> {
                            indicator = { width: 80.0 }
                        }
                        
                        <ShadecnProgress> {
                            indicator = { width: 200.0, draw_bg: { color: (COLOR_SUCCESS) } }
                        }
                    }
                }
            }
            
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnProgress> {\n    indicator = { width: 100.0 }\n}\n```"
                }
            }
        }
    }
}
