use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_time_picker::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::ComponentSection;
    use crate::components::button::ComponentSubsection;
    use crate::components::button::MarkdownSection;
    
    link shadecn_time_picker;
    use link::shadecn_time_picker::*;

    pub TimePickerShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Time Picker"
            }
            ComponentSectionDescription = {
                text: "A component that allows users to select a time."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The Time Picker component allows users to select a time.\n\n## Usage\n\nImport the time picker component:\n\n```rust\nuse makepad_shadecn_time_picker::*;\n```"
                }
            }

            time_picker_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic Time Picker"
                    }
                    ComponentSubsectionDescription = {
                        text: "A standard time picker component."
                    }
                }
                ComponentSubsectionDemo = {
                    time_picker_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        
                        <ShadecnTimePicker> {
                            width: Fit,
                            height: Fit,
                        }
                    }
                }
            }
        }
    }
}
