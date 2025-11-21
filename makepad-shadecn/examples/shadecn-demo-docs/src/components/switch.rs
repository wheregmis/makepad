use makepad_widgets::*;
use makepad_shadecn_switch::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub SwitchShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Switch"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A control that allows the user to toggle between checked and not checked."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Switches are used to toggle a setting on or off.\n\n## Usage\n\nImport the switch component from the `makepad-shadecn-switch` crate:\n\n```rust\nuse makepad_shadecn_switch::*;\n```"
                }
            }

            switch_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Example"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple switch with a label."
                    }
                }
                ComponentSubsectionDemo = {
                    switch_demo = <View> {
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
                            
                            <ShadecnSwitch> {
                                text: "Airplane Mode"
                            }
                        }

                        <View> {
                            width: Fit,
                            height: Fit,
                            flow: Right,
                            align: {y: 0.5},
                            spacing: (SPACE_2),
                            
                            <ShadecnSwitch> {
                                text: "Bluetooth"
                                active: true
                            }
                        }
                    }
                }
            }
        }
    }
}
