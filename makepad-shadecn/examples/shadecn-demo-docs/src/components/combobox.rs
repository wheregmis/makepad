use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_combobox::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    // Local doc wrapper mirroring ShadecnCombobox styles to avoid missing live links in hot reload
    pub DocCombobox = <DropDown> {
        width: Fill, height: Fit,
        padding: {left: (SPACE_3), right: (SPACE_3), top: (SPACE_2), bottom: (SPACE_2)}

        draw_bg: {
            instance hover: 0.0
            instance focus: 0.0
            instance open: 0.0

            color: (COLOR_BG_PRIMARY)
            border_color: (COLOR_BORDER_PRIMARY)
            border_radius: (RADIUS_MD)
            border_size: 1.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    self.border_size,
                    self.border_size,
                    self.rect_size.x - self.border_size * 2.0,
                    self.rect_size.y - self.border_size * 2.0,
                    self.border_radius
                );
                sdf.fill(self.color);
                sdf.stroke(self.border_color, self.border_size);
                return sdf.result;
            }
        }

        draw_text: {
            text_style: { font_size: (FONT_BASE) }
            color: (COLOR_FG_PRIMARY)
        }

        popup_menu: <PopupMenu> {}
    }
    
    pub ComboboxShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Combobox"
            }
            ComponentSectionDescription = {
                text: "Autocomplete input and command palette with a list of suggestions."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Combobox component combines a text input with a listbox, allowing users to filter options.\n\n## Usage\n\nImport the combobox component:\n\n```rust\nuse makepad_shadecn_combobox::*;\n```"
                }
            }

            combobox_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Combobox"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard combobox with a list of options."
                    }
                }
                ComponentSubsectionDemo = {
                    combobox_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: (SPACE_3),
                        
                        <DocCombobox> {
                            width: 200.0
                            labels: ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"]
                            values: [NextJs, SvelteKit, NuxtJs, Remix, Astro]
                        }
                    }
                }
            }
        }
    }
}
