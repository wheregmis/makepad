use makepad_widgets::*;
use makepad_shadecn_card::*;
use makepad_shadecn_core::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::components::button::*; // Import shared components like ComponentSection

    pub CardShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Card"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a card with header, content, and footer."
            }
        }
        ComponentSectionContent = {
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "Cards are used to group related information and actions. They typically contain a header, content, and footer.\n\n## Usage\n\nImport the card component from the `makepad-shadecn-card` crate:\n\n```rust\nuse makepad_shadecn_card::*;\n```"
                }
            }

            card_subsection = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Example"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A simple card with a title, description, content, and footer."
                    }
                }
                ComponentSubsectionDemo = {
                    card_demo = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        align: {x: 0.5, y: 0.0},
                        
                        <ShadecnCard> {
                            width: 350.0,
                            
                            <ShadecnCardHeader> {
                                <ShadecnCardTitle> { text: "Create project" }
                                <ShadecnCardDescription> { text: "Deploy your new project in one-click." }
                            }
                            
                            <ShadecnCardContent> {
                                flow: Down,
                                spacing: (SPACE_4),
                                
                                <Label> { text: "Name" }
                                <ShadecnInput> { 
                                    width: Fill,
                                    text: "Next.js Project" 
                                }
                                
                                <Label> { text: "Framework" }
                                <ShadecnInput> { 
                                    width: Fill,
                                    text: "Next.js" 
                                }
                            }
                            
                            <ShadecnCardFooter> {
                                flow: Right,
                                align: {x: 1.0, y: 0.5},
                                spacing: (SPACE_2),
                                
                                <ShadecnButtonOutline> { text: "Cancel" }
                                <ShadecnButton> { text: "Deploy" }
                            }
                        }
                    }
                }
            }
        }
    }
}
