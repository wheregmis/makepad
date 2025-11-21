use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_avatar::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    pub AvatarShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Avatar"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "An image element with a fallback for representing the user."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Avatar component is used to represent a user, and displays a profile picture, initials or fallback icon.\n\n## Usage\n\nImport from the `makepad-shadecn-avatar` crate:\n\n```rust\nuse makepad_shadecn_avatar::*;\n```"
                }
            }

            avatar_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Examples"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Avatar with fallback text."
                    }
                }
                ComponentSubsectionDemo = {
                    avatar_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right { wrap: true },
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        <ShadecnAvatar> {
                            <ShadecnAvatarFallback> {
                                label = { text: "CN" }
                            }
                        }

                        <ShadecnAvatar> {
                            <ShadecnAvatarFallback> {
                                label = { text: "JD" }
                                draw_bg: { color: (COLOR_SLATE_300) }
                            }
                        }
                    }
                }
            }

            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnAvatar> {\n    <ShadecnAvatarFallback> {\n        label = { text: \"CN\" }\n    }\n}\n```"
                }
            }
        }
    }
}
