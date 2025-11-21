use makepad_shadecn_core::*;
use makepad_shadecn_calendar::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub CalendarShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Calendar"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A date field component that allows users to enter and edit date."
            }
        }
        ComponentSectionContent = {
            calendar_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Select a date from the calendar."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnCalendar> {}
                }
            }
        }
    }
}
