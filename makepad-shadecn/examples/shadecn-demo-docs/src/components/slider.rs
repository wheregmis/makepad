use makepad_shadecn_core::*;
use makepad_shadecn_slider::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub SliderShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Slider"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "An input where the user selects a value from within a given range."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Sliders allow users to make selections from a range of values.\n\n## Usage\n\nImport the slider component from the `makepad-shadecn-slider` crate:\n\n```rust\nuse makepad_shadecn_slider::*;\n```"
                }
            }

            slider_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Example"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple slider."
                    }
                }
                ComponentSubsectionDemo = {
                    slider_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_4),

                        <View> {
                            width: 300.0,
                            height: Fit,
                            flow: Down,
                            spacing: (SPACE_2),

                            <Label> { text: "Volume" }
                            <ShadecnSlider> {
                                min: 0.0,
                                max: 100.0,
                                default: 50.0
                            }
                        }
                    }
                }
            }
        }
    }
}
