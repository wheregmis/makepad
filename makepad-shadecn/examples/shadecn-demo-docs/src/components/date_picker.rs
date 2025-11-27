use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_date_picker::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_date_picker;
    use link::shadecn_date_picker::*;

    pub DatePickerShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Date Picker"
            }
            ComponentSectionDescription = {
                text: "A date picker component with range and presets."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Date Picker allows users to select a date from a calendar.\n\n## Usage\n\nImport the date picker component:\n\n```rust\nuse makepad_shadecn_date_picker::*;\n```"
                }
            }

            date_picker_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Date Picker"
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple date picker."
                    }
                }
                ComponentSubsectionDemo = {
                    date_picker_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: (SPACE_3),
                        
                        <ShadecnDatePicker> {}
                    }
                }
            }
        }
    }
}
