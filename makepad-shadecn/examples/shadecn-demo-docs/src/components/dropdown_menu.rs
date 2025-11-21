use makepad_code_editor::code_view::CodeView;
use makepad_shadecn_core::*;
use makepad_shadecn_dropdown_menu::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use makepad_code_editor::code_view::CodeView;
    use crate::components::button::*;

    // DropdownMenu Component Showcase
    pub DropdownMenuShowcase = <ComponentSection> {
        section_header = {
            ComponentSectionTitle = {
                text: "Dropdown Menu"
                margin: {bottom: (SPACE_2)}
            }
            ComponentSectionDescription = {
                text: "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
            }
        }
        ComponentSectionContent = {
            // Markdown description section
            markdown_intro = <MarkdownSection> {
                markdown_content = {
                    body: "The DropdownMenu component is used to display a list of options that appear when a button is clicked. It provides a clean interface for selecting from multiple options.\n\n## Usage\n\nImport the dropdown menu component from the `makepad-shadecn-dropdown-menu` crate:\n\n```rust\nuse makepad_shadecn_dropdown_menu::*;\n```\n\n## Design Tokens\n\nThe dropdown menu follows the Shadecn design system:\n- **Padding**: `space_2` (8px) horizontal, `space_2` (8px) vertical\n- **Border Radius**: `radius_md` (4px)\n- **Font Size**: `font_base` (14px)\n- **Border**: 1px solid `border_primary`\n- **Focus State**: Border color changes to `primary` color"
                }
            }

            // Basic dropdown subsection
            dropdown_subsection_basic = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Basic"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "A basic dropdown menu with selectable options."
                    }
                }
                ComponentSubsectionDemo = {
                    dropdown_demo_basic = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        dropdown_menu_1 = <ShadecnDropdownMenu> {
                            width: Fit,
                            height: Fit,
                            labels: ["Option 1", "Option 2", "Option 3", "Option 4"]
                            values: [1, 2, 3, 4]
                            selected_item: 0
                        }
                    }
                }
            }

            // With many options subsection
            dropdown_subsection_many = <ComponentSubsection> {
                subsection_header = {
                    ComponentSubsectionName = {
                        text: "Many Options"
                        margin: {bottom: (SPACE_1)}
                    }
                    ComponentSubsectionDescription = {
                        text: "Dropdown menu with many options to demonstrate scrolling behavior."
                    }
                }
                ComponentSubsectionDemo = {
                    dropdown_demo_many = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: (SPACE_3),
                        padding: {left: (SPACE_4), top: (SPACE_4), right: (SPACE_4), bottom: (SPACE_4)},

                        dropdown_menu_many = <ShadecnDropdownMenu> {
                            width: Fit,
                            height: Fit,
                            labels: [
                                "Apple", "Banana", "Cherry", "Date", "Elderberry",
                                "Fig", "Grape", "Honeydew", "Kiwi", "Lemon",
                                "Mango", "Orange", "Papaya", "Quince", "Raspberry"
                            ]
                            values: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
                            selected_item: 0
                        }
                    }
                }
            }

            // Markdown code example section
            markdown_example = <MarkdownSection> {
                markdown_content = {
                    body: "\n## Example\n\n```rust\nuse makepad_shadecn_dropdown_menu::*;\n\nlive_design! {\n    pub MyDropdown = <ShadecnDropdownMenu> {\n        width: Fit,\n        height: Fit,\n        labels: [\"Option 1\", \"Option 2\", \"Option 3\"]\n        values: [1, 2, 3]\n        selected_item: 0\n    }\n}\n```\n\n## Props\n\n- `labels` (String array): Array of label strings to display in the dropdown\n- `values` (LiveId array): Array of values corresponding to each label\n- `selected_item` (int): Index of the currently selected item (0-based)\n- `width` (float): Width of the dropdown (Fit, Fill, or specific value)\n- `height` (float): Height of the dropdown (Fit, Fill, or specific value)\n\n## Styling\n\nThe dropdown menu uses the following design tokens:\n- Background: `COLOR_BG_PRIMARY`\n- Border: `COLOR_BORDER_PRIMARY` (1px)\n- Border Radius: `RADIUS_MD` (4px)\n- Text Color: `COLOR_FG_PRIMARY`\n- Hover Background: `COLOR_BG_HOVER`\n- Focus Border: `COLOR_PRIMARY`"
                }
            }
        }
    }
}
