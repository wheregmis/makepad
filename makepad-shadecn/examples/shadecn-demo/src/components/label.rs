use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_label::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    // Label Component Showcase
    pub LabelShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Label"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Renders an accessible label associated with controls."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Label component is used to display text labels with different styles and sizes. It follows the Shadecn design system for consistent typography.\n\n## Usage\n\nImport the label component from the `makepad-shadecn-label` crate:\n\n```rust\nuse makepad_shadecn_label::*;\n```"
                }
            }

            label_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Variants"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Label comes in different variants for different use cases."
                    }
                }
                ComponentSubsectionDemo = {
                    label_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        <ShadecnLabel> { text: "Primary Label" }
                        <ShadecnLabelSecondary> { text: "Secondary Label" }
                        <ShadecnLabelTertiary> { text: "Tertiary Label" }
                        <ShadecnLabelDisabled> { text: "Disabled Label" }
                        <ShadecnLabelSm> { text: "Small Label" }
                        <ShadecnLabelLg> { text: "Large Label" }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnLabel> { text: \"Primary Label\" }\n<ShadecnLabelSecondary> { text: \"Secondary Label\" }\n<ShadecnLabelTertiary> { text: \"Tertiary Label\" }\n<ShadecnLabelDisabled> { text: \"Disabled Label\" }\n<ShadecnLabelSm> { text: \"Small Label\" }\n<ShadecnLabelLg> { text: \"Large Label\" }\n```"
                }
            }
        }
    }
}
