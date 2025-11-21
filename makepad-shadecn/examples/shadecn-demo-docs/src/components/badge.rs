use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_badge::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    // Badge Component Showcase
    pub BadgeShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Badge"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a pill-style label for counts or statuses."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Badge highlights short bits of information like status or counts.\n\n## Usage\n\nImport from the `makepad-shadecn-badge` crate:\n\n```rust\nuse makepad_shadecn_badge::*;\n```"
                }
            }

            badge_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Variants"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Solid and outline pills sized with design tokens."
                    }
                }
                ComponentSubsectionDemo = {
                    badge_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right { wrap: true },
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        <ShadecnBadge> { label = { text: "Default" } }
                        <ShadecnBadgeSecondary> { label = { text: "Secondary" } }
                        <ShadecnBadgeDestructive> { label = { text: "Destructive" } }
                        <ShadecnBadgeOutline> { label = { text: "Outline" } }
                        <ShadecnBadgeMuted> { label = { text: "Muted" } }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnBadge> { label = { text: \"Badge\" } }\n<ShadecnBadgeSecondary> { label = { text: \"Secondary\" } }\n<ShadecnBadgeDestructive> { label = { text: \"Destructive\" } }\n<ShadecnBadgeOutline> { label = { text: \"Outline\" } }\n<ShadecnBadgeMuted> { label = { text: \"Muted\" } }\n```"
                }
            }
        }
    }
}
