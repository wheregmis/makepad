use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_skeleton::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    pub SkeletonShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Skeleton"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Use to show a placeholder while content is loading."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Skeleton is used to display a placeholder state while data is being fetched.\n\n## Usage\n\nImport from the `makepad-shadecn-skeleton` crate:\n\n```rust\nuse makepad_shadecn_skeleton::*;\n```"
                }
            }

            skeleton_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Examples"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Skeleton loading state for a card."
                    }
                }
                ComponentSubsectionDemo = {
                    skeleton_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: (SPACE_4),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},
                        align: {x: 0.0, y: 0.0},

                        <View> {
                            width: Fit,
                            height: Fit,
                            flow: Right,
                            spacing: (SPACE_4),
                            
                            <ShadecnSkeleton> {
                                width: 48.0,
                                height: 48.0,
                                draw_bg: { border_radius: (RADIUS_FULL) }
                            }
                            
                            <View> {
                                width: Fit,
                                height: Fit,
                                flow: Down,
                                spacing: (SPACE_2),
                                
                                <ShadecnSkeleton> {
                                    width: 250.0,
                                    height: 16.0,
                                }
                                
                                <ShadecnSkeleton> {
                                    width: 200.0,
                                    height: 16.0,
                                }
                            }
                        }
                    }
                }
            }
            
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnSkeleton> {\n    width: 100.0,\n    height: 20.0,\n}\n```"
                }
            }
        }
    }
}
