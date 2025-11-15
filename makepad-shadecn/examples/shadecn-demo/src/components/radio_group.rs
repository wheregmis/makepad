use makepad_shadecn_core::*;
use makepad_shadecn_radio_group::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    // RadioGroup Component Showcase
    pub RadioGroupShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "RadioGroup"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A set of checkable buttons—known as radio buttons—where no more than one of the buttons can be checked at a time."
            }
        }
        ComponentSectionContent = {
            radio_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A group of radio buttons where only one can be selected."
                    }
                }
                ComponentSubsectionDemo = {
                    radio_demo = <ShadecnRadioButtonGroup> {
                        width: Fit,
                        height: Fit,
                        spacing: (SPACE_3),
                        
                        radio_button_1 = <ShadecnRadioButton> {
                            text: "Option 1"
                        }
                        radio_button_2 = <ShadecnRadioButton> {
                            text: "Option 2"
                        }
                        radio_button_3 = <ShadecnRadioButton> {
                            text: "Option 3"
                        }
                    }
                }
            }
        }
    }
}
