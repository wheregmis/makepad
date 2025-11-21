use makepad_shadecn_core::*;
use makepad_shadecn_carousel::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*;

    pub CarouselShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Carousel"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "A carousel with motion and swipe built using Embla."
            }
        }
        ComponentSectionContent = {
            carousel_demo = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Swipe or use controls to navigate items."
                    }
                }
                ComponentSubsectionDemo = {
                    <ShadecnCarousel> {}
                }
            }
        }
    }
}
