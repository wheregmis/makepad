use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_input::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;
    
    link shadecn_input;
    use link::shadecn_input::*;

    // Input Component Showcase
    pub InputShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Input"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a form input field or a component that looks like an input field."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Input component is used to accept user input in forms and other interfaces. It provides a text field with styling that matches the Shadecn design system.\n\n## Usage\n\nImport the input component from the `makepad-shadecn-input` crate:\n\n```rust\nuse makepad_shadecn_input::*;\n```"
                }
            }

            input_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A basic input field with placeholder text."
                    }
                }
                ComponentSubsectionDemo = {
                    input_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        <ShadecnInput> {
                            empty_text: "Enter your email"
                        }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnInput> {\n    empty_text: \"Enter your email\"\n}\n```"
                }
            }
        }
    }
}
