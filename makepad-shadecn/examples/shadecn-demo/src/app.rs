use makepad_shadecn_core::*;
use makepad_shadecn_input::*;
use makepad_widgets::*;

use crate::layout;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::components::button::*;
    use crate::components::input::*;
    use crate::components::label::*;
    use crate::components::radio_group::*;
    use crate::components::dropdown_menu::*;
    use crate::components::scroll_area::*;
    use link::widgets::*;

    AppWindow = {{AppWindow}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    inner_size: vec2(1600.0, 1000.0),
                    position: vec2(50.0, 50.0),
                    title: "Shadecn UI Components - Documentation"
                }

                body = <View> {
                    width: Fill,
                    height: Fill,
                    show_bg: true,
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            return (COLOR_BG_PRIMARY);
                        }
                    }

                    layout = <AdaptiveView> {
                        Desktop = {
                            flow: Down,
                            spacing: 0,

                            header = <DocHeader> {}

                            main_layout = <View> {
                                width: Fill,
                                height: Fill,
                                flow: Right,
                                spacing: 0,

                                sidebar = <DocSidebar> {}

                                content_area = <DocContent> {
                                    content_scroll = {
                                        DocContentInner = {
                                            // Getting Started Page
                                            getting_started_page = <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: (SPACE_4),
                                                margin: {bottom: (SPACE_4)},
                                                visible: true,

                                                title = <Label> {
                                                    text: "Getting started",
                                                    margin: {bottom: (SPACE_3)},
                                                    draw_text: {
                                                        text_style: {
                                                            font_size: 36.0,
                                                        }
                                                        wrap: Word,
                                                        color: (COLOR_FG_PRIMARY)
                                                    }
                                                }

                                                subtitle = <Label> {
                                                    text: "Welcome to Shadcn UI for Makepad. This is the official documentation for Shadcn UI for Makepad.",
                                                    draw_text: {
                                                        text_style: {
                                                            font_size: (FONT_LG),
                                                        }
                                                        wrap: Word,
                                                        color: (COLOR_FG_SECONDARY)
                                                    }
                                                }
                                            }

                                            // Button Component Page
                                            button_page = <ButtonShowcase> {
                                                visible: false
                                            }

                                            // Input Component Page
                                            input_page = <InputShowcase> {
                                                visible: false
                                            }

                                            // Label Component Page
                                            label_page = <LabelShowcase> {
                                                visible: false
                                            }

                                            // RadioGroup Component Page
                                            radio_group_page = <RadioGroupShowcase> {
                                                visible: false
                                            }

                                            // DropdownMenu Component Page
                                            dropdown_menu_page = <DropdownMenuShowcase> {
                                                visible: false
                                            }

                                            // ScrollArea Component Page
                                            scroll_area_page = <ScrollAreaShowcase> {
                                                visible: false
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        Mobile = {
                            flow: Down,
                            spacing: 0,

                            header = <DocHeader> {}

                            content_mobile = <DocContent> {
                                content_scroll = {
                                    DocContentInner = {
                                        // Getting Started Page
                                        getting_started_page_mobile = <View> {
                                            width: Fill,
                                            height: Fit,
                                            flow: Down,
                                            spacing: (SPACE_4),
                                            margin: {bottom: (SPACE_4)},
                                            visible: true,

                                            title = <Label> {
                                                text: "Getting started",
                                                margin: {bottom: (SPACE_3)},
                                                draw_text: {
                                                    text_style: {
                                                        font_size: 28.0,
                                                    }
                                                    wrap: Word,
                                                    color: (COLOR_FG_PRIMARY)
                                                }
                                            }

                                            subtitle = <Label> {
                                                text: "Welcome to Shadcn UI for Makepad. This is the official documentation for Shadcn UI for Makepad.",
                                                draw_text: {
                                                    text_style: {
                                                        font_size: (FONT_BASE),
                                                    }
                                                    wrap: Word,
                                                    color: (COLOR_FG_SECONDARY)
                                                }
                                            }
                                        }

                                        // Button Component Page
                                        button_page_mobile = <ButtonShowcase> {
                                            visible: false
                                        }

                                        // Input Component Page
                                        input_page_mobile = <InputShowcase> {
                                            visible: false
                                        }

                                        // Label Component Page
                                        label_page_mobile = <LabelShowcase> {
                                            visible: false
                                        }

                                        // RadioGroup Component Page
                                        radio_group_page_mobile = <RadioGroupShowcase> {
                                            visible: false
                                        }

                                        // DropdownMenu Component Page
                                        dropdown_menu_page_mobile = <DropdownMenuShowcase> {
                                            visible: false
                                        }

                                        // ScrollArea Component Page
                                        scroll_area_page_mobile = <ScrollAreaShowcase> {
                                            visible: false
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

app_main!(AppWindow);

#[derive(Live)]
pub struct AppWindow {
    #[live]
    ui: WidgetRef,
    #[rust]
    search_query: String,
    #[rust]
    popup_item_clicked: bool,
}

impl LiveRegister for AppWindow {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_shadecn_core::live_design(cx);
        makepad_shadecn_button::live_design(cx);
        makepad_shadecn_input::live_design(cx);
        makepad_shadecn_label::live_design(cx);
        makepad_shadecn_radio_group::live_design(cx);
        makepad_shadecn_dropdown_menu::live_design(cx);
        makepad_shadecn_scroll_area::live_design(cx);
        makepad_code_editor::live_design(cx);
        layout::live_design(cx);
        crate::components::button::live_design(cx);
        crate::components::input::live_design(cx);
        crate::components::label::live_design(cx);
        crate::components::radio_group::live_design(cx);
        crate::components::dropdown_menu::live_design(cx);
        crate::components::scroll_area::live_design(cx);
    }
}

impl MatchEvent for AppWindow {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle navigation button clicks
        let ui = self.ui.clone();

        // Handle search input changes
        if let Some(search_text) = ui.text_input(ids!(search_area)).changed(actions) {
            self.search_query = search_text.clone();
            self.update_search_filter(cx);
        }

        // Handle search result clicks
        let ui2 = self.ui.clone();
        self.popup_item_clicked = false;

        if ui2.button(ids!(search_result_button)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "button");
        }
        if ui2.button(ids!(search_result_input)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "input");
        }
        if ui2.button(ids!(search_result_label)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "label");
        }
        if ui2.button(ids!(search_result_radio_group)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "radio_group");
        }
        if ui2
            .button(ids!(search_result_dropdown_menu))
            .clicked(actions)
        {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "dropdown_menu");
        }
        if ui2.button(ids!(search_result_scroll_area)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "scroll_area");
        }

        // Handle radio button group selection (make them mutually exclusive)
        ui.radio_button_set(ids_array!(
            radio_group_page
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_1,
            radio_group_page
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_2,
            radio_group_page
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_3
        ))
        .selected(cx, actions);

        // Also handle mobile version
        ui.radio_button_set(ids_array!(
            radio_group_page_mobile
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_1,
            radio_group_page_mobile
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_2,
            radio_group_page_mobile
                .radio_subsection
                .ComponentSubsectionDemo
                .radio_demo
                .radio_button_3
        ))
        .selected(cx, actions);

        // Hide popup when clicking outside or losing focus (unless a popup item was clicked)
        if !self.popup_item_clicked {
            for action in actions.iter() {
                if let Some(text_action) = action.as_widget_action() {
                    if text_action.widget_uid == ui.text_input(ids!(search_area)).widget_uid() {
                        if let Some(TextInputAction::KeyFocusLost) = text_action.cast() {
                            ui.view(ids!(search_results_popup)).set_visible(cx, false);
                        }
                    }
                }
            }
        }

        // Navigation to Getting Started / Overview
        if ui.button(ids!(nav_link_getting_started)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, true);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, true);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to Button component
        if ui.button(ids!(component_link_button)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, true);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, true);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to Input component
        if ui.button(ids!(component_link_input)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, true);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, true);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to Label component
        if ui.button(ids!(component_link_label)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, true);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, true);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to RadioGroup component
        if ui.button(ids!(component_link_radio_group)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, true);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile)).set_visible(cx, true);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to DropdownMenu component
        if ui
            .button(ids!(component_link_dropdown_menu))
            .clicked(actions)
        {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, true);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, true);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
        }

        // Navigation to ScrollArea component
        if ui.button(ids!(component_link_scroll_area)).clicked(actions) {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, true);
            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile)).set_visible(cx, true);
        }
    }
}

impl AppWindow {
    fn update_search_filter(&mut self, cx: &mut Cx) {
        let ui = self.ui.clone();
        let query = self.search_query.to_lowercase();

        // Component search terms and their result buttons and navigation actions
        let components = vec![
            ("button", ids!(search_result_button), "button"),
            ("input", ids!(search_result_input), "input"),
            ("textinput", ids!(search_result_input), "input"),
            ("label", ids!(search_result_label), "label"),
            ("text", ids!(search_result_label), "label"),
            ("radio", ids!(search_result_radio_group), "radio_group"),
            ("radiogroup", ids!(search_result_radio_group), "radio_group"),
            (
                "radio button",
                ids!(search_result_radio_group),
                "radio_group",
            ),
            (
                "dropdown",
                ids!(search_result_dropdown_menu),
                "dropdown_menu",
            ),
            ("menu", ids!(search_result_dropdown_menu), "dropdown_menu"),
            (
                "dropdown menu",
                ids!(search_result_dropdown_menu),
                "dropdown_menu",
            ),
            ("scroll", ids!(search_result_scroll_area), "scroll_area"),
            ("area", ids!(search_result_scroll_area), "scroll_area"),
            (
                "scroll area",
                ids!(search_result_scroll_area),
                "scroll_area",
            ),
            ("scrollarea", ids!(search_result_scroll_area), "scroll_area"),
        ];

        // Get all result button IDs
        let all_result_ids = vec![
            ids!(search_result_button),
            ids!(search_result_input),
            ids!(search_result_label),
            ids!(search_result_radio_group),
            ids!(search_result_dropdown_menu),
            ids!(search_result_scroll_area),
        ];

        if query.is_empty() {
            // Hide popup when search is empty
            ui.view(ids!(search_results_popup)).set_visible(cx, false);
        } else {
            // Show popup and filter results
            ui.view(ids!(search_results_popup)).set_visible(cx, true);

            // Hide all results first
            for result_id in &all_result_ids {
                ui.button(*result_id).set_visible(cx, false);
            }

            // Show matching results
            let mut has_results = false;
            for (name, result_id, _) in &components {
                if name.contains(&query) {
                    ui.button(*result_id).set_visible(cx, true);
                    has_results = true;
                }
            }

            // Hide popup if no results
            if !has_results {
                ui.view(ids!(search_results_popup)).set_visible(cx, false);
            }
        }
    }

    fn navigate_to_component(&mut self, cx: &mut Cx, component: &str) {
        let ui = self.ui.clone();

        // Hide popup
        ui.view(ids!(search_results_popup)).set_visible(cx, false);

        // Clear search
        self.search_query = String::new();
        ui.text_input(ids!(search_area)).set_text(cx, "");

        // Navigate based on component name
        match component {
            "button" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, true);
                ui.view(ids!(input_page)).set_visible(cx, false);
                ui.view(ids!(label_page)).set_visible(cx, false);
                ui.view(ids!(radio_group_page)).set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
                ui.view(ids!(scroll_area_page)).set_visible(cx, false);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, true);
                ui.view(ids!(input_page_mobile)).set_visible(cx, false);
                ui.view(ids!(label_page_mobile)).set_visible(cx, false);
                ui.view(ids!(radio_group_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(scroll_area_page_mobile))
                    .set_visible(cx, false);
            }
            "input" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, false);
                ui.view(ids!(input_page)).set_visible(cx, true);
                ui.view(ids!(label_page)).set_visible(cx, false);
                ui.view(ids!(radio_group_page)).set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
                ui.view(ids!(scroll_area_page)).set_visible(cx, false);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, false);
                ui.view(ids!(input_page_mobile)).set_visible(cx, true);
                ui.view(ids!(label_page_mobile)).set_visible(cx, false);
                ui.view(ids!(radio_group_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(scroll_area_page_mobile))
                    .set_visible(cx, false);
            }
            "label" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, false);
                ui.view(ids!(input_page)).set_visible(cx, false);
                ui.view(ids!(label_page)).set_visible(cx, true);
                ui.view(ids!(radio_group_page)).set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
                ui.view(ids!(scroll_area_page)).set_visible(cx, false);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, false);
                ui.view(ids!(input_page_mobile)).set_visible(cx, false);
                ui.view(ids!(label_page_mobile)).set_visible(cx, true);
                ui.view(ids!(radio_group_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(scroll_area_page_mobile))
                    .set_visible(cx, false);
            }
            "radio_group" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, false);
                ui.view(ids!(input_page)).set_visible(cx, false);
                ui.view(ids!(label_page)).set_visible(cx, false);
                ui.view(ids!(radio_group_page)).set_visible(cx, true);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
                ui.view(ids!(scroll_area_page)).set_visible(cx, false);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, false);
                ui.view(ids!(input_page_mobile)).set_visible(cx, false);
                ui.view(ids!(label_page_mobile)).set_visible(cx, false);
                ui.view(ids!(radio_group_page_mobile)).set_visible(cx, true);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(scroll_area_page_mobile))
                    .set_visible(cx, false);
            }
            "dropdown_menu" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, false);
                ui.view(ids!(input_page)).set_visible(cx, false);
                ui.view(ids!(label_page)).set_visible(cx, false);
                ui.view(ids!(radio_group_page)).set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, true);
                ui.view(ids!(scroll_area_page)).set_visible(cx, false);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, false);
                ui.view(ids!(input_page_mobile)).set_visible(cx, false);
                ui.view(ids!(label_page_mobile)).set_visible(cx, false);
                ui.view(ids!(radio_group_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, true);
                ui.view(ids!(scroll_area_page_mobile))
                    .set_visible(cx, false);
            }
            "scroll_area" => {
                ui.view(ids!(getting_started_page)).set_visible(cx, false);
                ui.view(ids!(button_page)).set_visible(cx, false);
                ui.view(ids!(input_page)).set_visible(cx, false);
                ui.view(ids!(label_page)).set_visible(cx, false);
                ui.view(ids!(radio_group_page)).set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
                ui.view(ids!(scroll_area_page)).set_visible(cx, true);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(button_page_mobile)).set_visible(cx, false);
                ui.view(ids!(input_page_mobile)).set_visible(cx, false);
                ui.view(ids!(label_page_mobile)).set_visible(cx, false);
                ui.view(ids!(radio_group_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, false);
                ui.view(ids!(scroll_area_page_mobile)).set_visible(cx, true);
            }
            _ => {}
        }
    }
}

impl LiveHook for AppWindow {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.search_query = String::new();
        self.popup_item_clicked = false;
    }
}

impl AppMain for AppWindow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
