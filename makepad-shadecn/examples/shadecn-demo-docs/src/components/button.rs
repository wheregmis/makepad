use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_button::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;

    // Documentation component that can mix markdown and component demos
    pub DocContentWithMarkdown = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_4),
    }

    pub MarkdownSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_3),

        markdown_content = <Markdown> {
            width: Fill,
            height: Fit,
            flow: Right { wrap: true },
            padding: {left: 0, top: 0, right: 0, bottom: 0},

            font_size: (FONT_BASE),
            font_color: (COLOR_FG_PRIMARY),
            paragraph_spacing: (SPACE_4),
            pre_code_spacing: (SPACE_2),
            heading_base_scale: 1.8,
            use_code_block_widget: true,

            code_block = <View> {
                width: Fill,
                height: Fit,
                flow: Overlay,
                padding: {left: (SPACE_4), top: (SPACE_3), right: (SPACE_4), bottom: (SPACE_3)},
                margin: {top: (SPACE_2), bottom: (SPACE_2)},
                show_bg: true,
                draw_bg: {
                    uniform border_size: 1.0,
                    uniform border_radius: (RADIUS_MD),
                    color: (COLOR_BG_SECONDARY),
                    uniform border_color: (COLOR_BORDER_SECONDARY),
                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(
                            self.border_size,
                            self.border_size,
                            self.rect_size.x - (self.border_size * 2.0),
                            self.rect_size.y - (self.border_size * 2.0),
                            max(1.0, self.border_radius)
                        );
                        sdf.fill_keep(self.color);
                        if self.border_size > 0.0 {
                            sdf.stroke(self.border_color, self.border_size);
                        }
                        return sdf.result;
                    }
                }

                code_view = <CodeView> {
                    keep_cursor_at_end: true,
                    editor: {
                        height: Fit,
                        read_only: true,
                        show_gutter: false,
                        draw_bg: {
                            fn pixel(self) -> vec4 {
                                return vec4(0.0, 0.0, 0.0, 0.0);
                            }
                        }
                        draw_text: {
                            text_style: {
                                font_size: (FONT_SM),
                            }
                        }
                        token_colors: {
                            whitespace: (COLOR_FG_DISABLED),
                            delimiter: (COLOR_FG_TERTIARY),
                            delimiter_highlight: (COLOR_FG_PRIMARY),
                            error_decoration: (COLOR_ERROR),
                            warning_decoration: (COLOR_WARNING),

                            unknown: (COLOR_FG_PRIMARY),
                            branch_keyword: vec4(0.576, 0.200, 0.918, 1.0),
                            constant: (COLOR_ERROR),
                            identifier: (COLOR_FG_PRIMARY),
                            loop_keyword: vec4(0.918, 0.345, 0.047, 1.0),
                            number: vec4(0.086, 0.639, 0.290, 1.0),
                            other_keyword: (COLOR_ACCENT),
                            punctuator: (COLOR_FG_TERTIARY),
                            string: vec4(0.792, 0.541, 0.016, 1.0),
                            function: vec4(0.031, 0.569, 0.698, 1.0),
                            typename: vec4(0.020, 0.588, 0.412, 1.0),
                            comment: (COLOR_FG_DISABLED),
                        }
                    }
                }
            }

            draw_normal: {
                text_style: {
                    font_size: (FONT_BASE),
                }
                color: (COLOR_FG_PRIMARY),
            }

            draw_bold: {
                text_style: {
                    font_size: (FONT_BASE),
                }
                color: (COLOR_FG_PRIMARY),
            }

            draw_italic: {
                text_style: {
                    font_size: (FONT_BASE),
                }
                color: (COLOR_FG_PRIMARY),
            }

            draw_fixed: {
                text_style: {
                    font_size: (FONT_SM),
                }
                color: (COLOR_FG_SECONDARY),
            }
        }
    }

    // Reusable Component Section
    pub ComponentSection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: (SPACE_5),
        padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_6)},
        margin: {top: (SPACE_2), bottom: (SPACE_2)},
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG_PRIMARY);
            }
        }

        section_header = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: (SPACE_3),
            margin: {bottom: (SPACE_2)},

            ComponentSectionTitle = <Label> {
                text: "Section Title",
                margin: {bottom: (SPACE_2)},
                draw_text: {
                    text_style: {
                        font_size: (FONT_2XL),
                    }
                    color: (COLOR_FG_PRIMARY)
                }
            }

            ComponentSectionDescription = <Label> {
                text: "Section description text.",
                draw_text: {
                    text_style: {
                        font_size: (FONT_LG),
                    }
                    wrap: Word,
                    color: (COLOR_FG_SECONDARY)
                }
            }
        }

        ComponentSectionContent = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: (SPACE_6),
        }
    }

    // Reusable Component Subsection
    pub ComponentSubsection = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
            spacing: (SPACE_4),

        subsection_header = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: (SPACE_2),
            margin: {bottom: (SPACE_2)},

            ComponentSubsectionName = <Label> {
                text: "Component Name",
                margin: {bottom: (SPACE_1)},
                draw_text: {
                    text_style: {
                        font_size: (FONT_XL),
                    }
                    color: (COLOR_FG_SECONDARY)
                }
            }

            ComponentSubsectionDescription = <Label> {
                text: "Component description.",
                draw_text: {
                    text_style: {
                        font_size: (FONT_BASE),
                    }
                    wrap: Word,
                    color: (COLOR_FG_TERTIARY)
                }
            }
        }

        ComponentSubsectionDemo = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: (SPACE_3),
            padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_6)},
            show_bg: true,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_BG_SECONDARY);
                }
            }
        }
    }

    // Button Component Showcase
    pub ButtonShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Button"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a button or a component that looks like a button."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Button component is used to trigger an action or event, such as submitting a form, opening a dialog, canceling an action, or performing a delete operation.\n\n## Usage\n\nImport the button component from the `makepad-shadecn-button` crate:\n\n```rust\nuse makepad_shadecn_button::*;\n```"
                }
            }

            button_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Variants"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Button comes in multiple variants for different use cases."
                    }
                }
                ComponentSubsectionDemo = {
                    button_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right { wrap: true },
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        <ShadecnButton> { text: "Primary", width: Fit, height: Fit }
                        <ShadecnButtonSecondary> { text: "Secondary", width: Fit, height: Fit }
                        <ShadecnButtonOutline> { text: "Outline", width: Fit, height: Fit }
                        <ShadecnButtonDestructive> { text: "Destructive", width: Fit, height: Fit }
                        <ShadecnButtonGhost> { text: "Ghost", width: Fit, height: Fit }
                        <ShadecnButtonIcon> {
                            text: "",
                            width: Fit,
                            height: Fit
                        }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\n<ShadecnButton> { text: \"Click me\" }\n<ShadecnButtonSecondary> { text: \"Secondary\" }\n<ShadecnButtonOutline> { text: \"Outline\" }\n```"
                }
            }
        }
    }
}
