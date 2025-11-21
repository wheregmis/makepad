use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_separator::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    // Separator Component Showcase
    pub SeparatorShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Separator"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Thin dividers for grouping content horizontally or vertically."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Use Separator to create subtle dividers between sections.\n\n## Usage\n\nImport from the `makepad-shadecn-separator` crate:\n\n```rust\nuse makepad_shadecn_separator::*;\n```"
                }
            }

            separator_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Examples"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Horizontal and vertical variants sized with design tokens."
                    }
                }
                ComponentSubsectionDemo = {
                    separator_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_4),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        row = <View> {
                            width: Fill,
                            height: Fit,
                            flow: Right,
                            spacing: (SPACE_3),
                            align: { y: 0.5 },

                            <ShadecnLabel> { text: "Section A" }
                            <ShadecnSeparatorVertical> {}
                            <ShadecnLabel> { text: "Section B" }
                            <ShadecnSeparatorVertical> {}
                            <ShadecnLabel> { text: "Section C" }
                        }

                        <ShadecnSeparator> {}

                        column = <View> {
                            width: Fill,
                            height: Fit,
                            flow: Down,
                            spacing: (SPACE_2),

                            <ShadecnLabel> { text: "Item 1" }
                            <ShadecnSeparator> {}
                            <ShadecnLabel> { text: "Item 2" }
                            <ShadecnSeparator> {}
                            <ShadecnLabel> { text: "Item 3" }
                        }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnSeparator> {}\n<ShadecnSeparatorVertical> {}\n```"
                }
            }
        }
    }
}
