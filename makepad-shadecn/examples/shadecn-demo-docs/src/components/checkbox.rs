use makepad_widgets::*;
use makepad_shadecn_checkbox::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub CheckboxShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Checkbox"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A control that allows the user to toggle between checked and not checked."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Checkboxes are used when there are multiple items to select in a list. Users can select zero, one, or any number of items.\n\n## Usage\n\nImport the checkbox component from the `makepad-shadecn-checkbox` crate:\n\n```rust\nuse makepad_shadecn_checkbox::*;\n```"
                }
            }

            checkbox_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Example"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple checkbox with a label."
                    }
                }
                ComponentSubsectionDemo = {
                    checkbox_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_4),
                        
                        <View> {
                            width: Fit,
                            height: Fit,
                            flow: Right,
                            align: {y: 0.5},
                            spacing: (SPACE_2),
                            
                            <ShadecnCheckbox> {
                                text: "Accept terms and conditions"
                            }
                        }

                        <View> {
                            width: Fit,
                            height: Fit,
                            flow: Right,
                            align: {y: 0.5},
                            spacing: (SPACE_2),
                            
                            <ShadecnCheckbox> {
                                text: "Subscribe to newsletter"
                                active: true
                            }
                        }
                    }
                }
            }
        }
    }
}
