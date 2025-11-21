use makepad_shadecn_core::*;
use makepad_shadecn_alert::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub AlertShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Alert"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a callout for user attention."
            }
        }
        ComponentSectionContent = {
            alert_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Variants"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Default and destructive alert styles."
                    }
                }
                ComponentSubsectionDemo = {
                    <View> {
                        width: Fill, height: Fit
                        flow: Down
                        spacing: (SPACE_3)
                        
                        <ShadecnAlert> {}
                        <ShadecnAlertDestructive> {}
                    }
                }
            }
        }
    }
}
