use makepad_shadecn_avatar::*;
use makepad_shadecn_card::*;
use makepad_shadecn_checkbox::*;
use makepad_shadecn_combobox::*;
use makepad_shadecn_core::*;
use makepad_shadecn_date_picker::*;
use makepad_shadecn_input::*;
use makepad_shadecn_progress::*;
use makepad_shadecn_resizable::*;
use makepad_shadecn_skeleton::*;
use makepad_shadecn_slider::*;
use makepad_shadecn_switch::*;
use makepad_shadecn_time_picker::*;
use makepad_widgets::*;

use crate::layout;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::components::button::*;
    use crate::components::badge::*;
    use crate::components::input::*;
    use crate::components::label::*;
    use crate::components::radio_group::*;
    use crate::components::dropdown_menu::*;
    use crate::components::scroll_area::*;
    use crate::components::separator::*;
    use crate::components::card::*;
    use crate::components::checkbox::*;
    use crate::components::switch::*;
    use crate::components::slider::*;
    use crate::components::avatar::*;
    use crate::components::skeleton::*;
    use crate::components::progress::*;
    use crate::components::combobox::*;
    use crate::components::date_picker::*;
    use crate::components::menubar::*;
    use crate::components::navigation_menu::*;
    use crate::components::pagination::*;
    use crate::components::resizable::*;
    use crate::components::tabs::*;
    use crate::components::time_picker::*;
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

                                            // Badge Component Page
                                            badge_page = <BadgeShowcase> {
                                                visible: false
                                            }

                                            // Separator Component Page
                                            separator_page = <SeparatorShowcase> {
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

                                            // Card Component Page
                                            card_page = <CardShowcase> {
                                                visible: false
                                            }

                                            // Checkbox Component Page
                                            checkbox_page = <CheckboxShowcase> {
                                                visible: false
                                            }

                                            // Switch Component Page
                                            switch_page = <SwitchShowcase> {
                                                visible: false
                                            }

                                            // Slider Component Page
                                            slider_page = <SliderShowcase> {
                                                visible: false
                                            }

                                            // Avatar Component Page
                                            avatar_page = <AvatarShowcase> {
                                                visible: false
                                            }

                                            // Skeleton Component Page
                                            skeleton_page = <SkeletonShowcase> {
                                                visible: false
                                            }

                                            // Progress Component Page
                                            progress_page = <ProgressShowcase> {
                                                visible: false
                                            }

                                            // Combobox Component Page
                                            combobox_page = <ComboboxShowcase> {
                                                visible: false
                                            }

                                            // Date Picker Component Page
                                            date_picker_page = <DatePickerShowcase> {
                                                visible: false
                                            }

                                            // Menubar Component Page
                                            menubar_page = <MenubarShowcase> {
                                                visible: false
                                            }

                                            // Navigation Menu Component Page
                                            navigation_menu_page = <NavigationMenuShowcase> {
                                                visible: false
                                            }

                                            // Pagination Component Page
                                            pagination_page = <PaginationShowcase> {
                                                visible: false
                                            }

                                            // Resizable Component Page
                                            resizable_page = <ResizableShowcase> {
                                                visible: false
                                            }

                                            // Tabs Component Page
                                            tabs_page = <TabsShowcase> {
                                                visible: false
                                            }

                                            // Time Picker Component Page
                                            time_picker_page = <TimePickerShowcase> {
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

                                        // Badge Component Page
                                        badge_page_mobile = <BadgeShowcase> {
                                            visible: false
                                        }

                                        // Separator Component Page
                                        separator_page_mobile = <SeparatorShowcase> {
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

                                        // Card Component Page
                                        card_page_mobile = <CardShowcase> {
                                            visible: false
                                        }

                                        // Checkbox Component Page
                                        checkbox_page_mobile = <CheckboxShowcase> {
                                            visible: false
                                        }

                                        // Switch Component Page
                                        switch_page_mobile = <SwitchShowcase> {
                                            visible: false
                                        }

                                        // Slider Component Page
                                        slider_page_mobile = <SliderShowcase> {
                                            visible: false
                                        }

                                        // Avatar Component Page
                                        avatar_page_mobile = <AvatarShowcase> {
                                            visible: false
                                        }

                                        // Skeleton Component Page
                                        skeleton_page_mobile = <SkeletonShowcase> {
                                            visible: false
                                        }

                                        // Progress Component Page
                                        progress_page_mobile = <ProgressShowcase> {
                                            visible: false
                                        }

                                        // Combobox Component Page
                                        combobox_page_mobile = <ComboboxShowcase> {
                                            visible: false
                                        }

                                        // Date Picker Component Page
                                        date_picker_page_mobile = <DatePickerShowcase> {
                                            visible: false
                                        }

                                        // Menubar Component Page
                                        menubar_page_mobile = <MenubarShowcase> {
                                            visible: false
                                        }

                                        // Navigation Menu Component Page
                                        navigation_menu_page_mobile = <NavigationMenuShowcase> {
                                            visible: false
                                        }

                                        // Pagination Component Page
                                        pagination_page_mobile = <PaginationShowcase> {
                                            visible: false
                                        }

                                        // Resizable Component Page
                                        resizable_page_mobile = <ResizableShowcase> {
                                            visible: false
                                        }

                                        // Tabs Component Page
                                        tabs_page_mobile = <TabsShowcase> {
                                            visible: false
                                        }

                                        // Time Picker Component Page
                                        time_picker_page_mobile = <TimePickerShowcase> {
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
        makepad_shadecn_badge::live_design(cx);
        makepad_shadecn_input::live_design(cx);
        makepad_shadecn_label::live_design(cx);
        makepad_shadecn_radio_group::live_design(cx);
        makepad_shadecn_dropdown_menu::live_design(cx);
        makepad_shadecn_separator::live_design(cx);
        makepad_shadecn_scroll_area::live_design(cx);
        makepad_shadecn_card::live_design(cx);
        makepad_shadecn_checkbox::live_design(cx);
        makepad_shadecn_switch::live_design(cx);
        makepad_shadecn_slider::live_design(cx);
        makepad_shadecn_avatar::live_design(cx);
        makepad_shadecn_skeleton::live_design(cx);
        makepad_shadecn_progress::live_design(cx);
        makepad_shadecn_combobox::live_design(cx);
        makepad_shadecn_date_picker::live_design(cx);
        makepad_shadecn_time_picker::live_design(cx);
        makepad_code_editor::live_design(cx);
        layout::live_design(cx);
        crate::components::button::live_design(cx);
        crate::components::badge::live_design(cx);
        crate::components::input::live_design(cx);
        crate::components::label::live_design(cx);
        crate::components::separator::live_design(cx);
        crate::components::radio_group::live_design(cx);
        crate::components::dropdown_menu::live_design(cx);
        crate::components::scroll_area::live_design(cx);
        crate::components::card::live_design(cx);
        crate::components::checkbox::live_design(cx);
        crate::components::switch::live_design(cx);
        crate::components::slider::live_design(cx);
        crate::components::avatar::live_design(cx);
        crate::components::skeleton::live_design(cx);
        crate::components::progress::live_design(cx);
        makepad_shadecn_tabs::live_design(cx);
        makepad_shadecn_navigation_menu::live_design(cx);
        makepad_shadecn_pagination::live_design(cx);
        makepad_shadecn_menubar::live_design(cx);
        makepad_shadecn_resizable::live_design(cx);
        crate::components::combobox::live_design(cx);
        crate::components::date_picker::live_design(cx);
        crate::components::menubar::live_design(cx);
        crate::components::navigation_menu::live_design(cx);
        crate::components::pagination::live_design(cx);
        crate::components::resizable::live_design(cx);
        crate::components::tabs::live_design(cx);
        crate::components::time_picker::live_design(cx);
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
        if ui2.button(ids!(search_result_badge)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "badge");
        }
        if ui2.button(ids!(search_result_separator)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "separator");
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
        if ui2.button(ids!(search_result_avatar)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "avatar");
        }
        if ui2.button(ids!(search_result_skeleton)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "skeleton");
        }
        if ui2.button(ids!(search_result_progress)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "progress");
        }
        if ui2.button(ids!(search_result_combobox)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "combobox");
        }
        if ui2.button(ids!(search_result_date_picker)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "date_picker");
        }
        if ui2.button(ids!(search_result_menubar)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "menubar");
        }
        if ui2
            .button(ids!(search_result_navigation_menu))
            .clicked(actions)
        {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "navigation_menu");
        }
        if ui2.button(ids!(search_result_pagination)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "pagination");
        }
        if ui2.button(ids!(search_result_resizable)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "resizable");
        }
        if ui2.button(ids!(search_result_tabs)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "tabs");
        }
        if ui2.button(ids!(search_result_time_picker)).clicked(actions) {
            self.popup_item_clicked = true;
            self.navigate_to_component(cx, "time_picker");
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
            self.navigate_to_component(cx, "getting_started");
        }

        // Navigation to Button component
        if ui.button(ids!(component_link_button)).clicked(actions) {
            self.navigate_to_component(cx, "button");
        }

        // Navigation to Input component
        if ui.button(ids!(component_link_input)).clicked(actions) {
            self.navigate_to_component(cx, "input");
        }

        // Navigation to Label component
        if ui.button(ids!(component_link_label)).clicked(actions) {
            self.navigate_to_component(cx, "label");
        }

        // Navigation to Badge component
        if ui.button(ids!(component_link_badge)).clicked(actions) {
            self.navigate_to_component(cx, "badge");
        }

        // Navigation to Separator component
        if ui.button(ids!(component_link_separator)).clicked(actions) {
            self.navigate_to_component(cx, "separator");
        }

        // Navigation to RadioGroup component
        if ui.button(ids!(component_link_radio_group)).clicked(actions) {
            self.navigate_to_component(cx, "radio_group");
        }

        // Navigation to DropdownMenu component
        if ui
            .button(ids!(component_link_dropdown_menu))
            .clicked(actions)
        {
            self.navigate_to_component(cx, "dropdown_menu");
        }

        // Navigation to ScrollArea component
        if ui.button(ids!(component_link_scroll_area)).clicked(actions) {
            self.navigate_to_component(cx, "scroll_area");
        }

        // Navigation to Card component
        if ui.button(ids!(component_link_card)).clicked(actions) {
            self.navigate_to_component(cx, "card");
        }

        // Navigation to Checkbox component
        if ui.button(ids!(component_link_checkbox)).clicked(actions) {
            self.navigate_to_component(cx, "checkbox");
        }

        // Navigation to Switch component
        if ui.button(ids!(component_link_switch)).clicked(actions) {
            self.navigate_to_component(cx, "switch");
        }

        // Navigation to Slider component
        if ui.button(ids!(component_link_slider)).clicked(actions) {
            self.navigate_to_component(cx, "slider");
        }

        // Navigation to Avatar component
        if ui.button(ids!(component_link_avatar)).clicked(actions) {
            self.navigate_to_component(cx, "avatar");
        }

        // Navigation to Skeleton component
        if ui.button(ids!(component_link_skeleton)).clicked(actions) {
            self.navigate_to_component(cx, "skeleton");
        }

        // Navigation to Progress component
        if ui.button(ids!(component_link_progress)).clicked(actions) {
            self.navigate_to_component(cx, "progress");
        }

        // Navigation to Combobox component
        if ui.button(ids!(component_link_combobox)).clicked(actions) {
            self.navigate_to_component(cx, "combobox");
        }

        // Navigation to Date Picker component
        if ui.button(ids!(component_link_date_picker)).clicked(actions) {
            self.navigate_to_component(cx, "date_picker");
        }

        // Navigation to Menubar component
        if ui.button(ids!(component_link_menubar)).clicked(actions) {
            self.navigate_to_component(cx, "menubar");
        }

        // Navigation to Navigation Menu component
        if ui
            .button(ids!(component_link_navigation_menu))
            .clicked(actions)
        {
            self.navigate_to_component(cx, "navigation_menu");
        }

        // Navigation to Pagination component
        if ui.button(ids!(component_link_pagination)).clicked(actions) {
            self.navigate_to_component(cx, "pagination");
        }

        // Navigation to Resizable component
        if ui.button(ids!(component_link_resizable)).clicked(actions) {
            self.navigate_to_component(cx, "resizable");
        }

        // Navigation to Tabs component
        if ui.button(ids!(component_link_tabs)).clicked(actions) {
            self.navigate_to_component(cx, "tabs");
        }

        // Navigation to Time Picker component
        if ui.button(ids!(component_link_time_picker)).clicked(actions) {
            self.navigate_to_component(cx, "time_picker");
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
            ("badge", ids!(search_result_badge), "badge"),
            ("pill", ids!(search_result_badge), "badge"),
            ("separator", ids!(search_result_separator), "separator"),
            ("divider", ids!(search_result_separator), "separator"),
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
            ("card", ids!(search_result_card), "card"),
            ("checkbox", ids!(search_result_checkbox), "checkbox"),
            ("check", ids!(search_result_checkbox), "checkbox"),
            ("switch", ids!(search_result_switch), "switch"),
            ("toggle", ids!(search_result_switch), "switch"),
            ("slider", ids!(search_result_slider), "slider"),
            ("range", ids!(search_result_slider), "slider"),
            ("avatar", ids!(search_result_avatar), "avatar"),
            ("image", ids!(search_result_avatar), "avatar"),
            ("profile", ids!(search_result_avatar), "avatar"),
            ("skeleton", ids!(search_result_skeleton), "skeleton"),
            ("loading", ids!(search_result_skeleton), "skeleton"),
            ("placeholder", ids!(search_result_skeleton), "skeleton"),
            ("progress", ids!(search_result_progress), "progress"),
            ("bar", ids!(search_result_progress), "progress"),
            ("loading bar", ids!(search_result_progress), "progress"),
            ("combobox", ids!(search_result_combobox), "combobox"),
            (
                "date picker",
                ids!(search_result_date_picker),
                "date_picker",
            ),
            ("menubar", ids!(search_result_menubar), "menubar"),
            (
                "navigation menu",
                ids!(search_result_navigation_menu),
                "navigation_menu",
            ),
            ("pagination", ids!(search_result_pagination), "pagination"),
            ("resizable", ids!(search_result_resizable), "resizable"),
            ("tabs", ids!(search_result_tabs), "tabs"),
            (
                "time picker",
                ids!(search_result_time_picker),
                "time_picker",
            ),
        ];

        // Get all result button IDs
        let all_result_ids = vec![
            ids!(search_result_button),
            ids!(search_result_input),
            ids!(search_result_label),
            ids!(search_result_badge),
            ids!(search_result_separator),
            ids!(search_result_radio_group),
            ids!(search_result_dropdown_menu),
            ids!(search_result_scroll_area),
            ids!(search_result_card),
            ids!(search_result_checkbox),
            ids!(search_result_switch),
            ids!(search_result_slider),
            ids!(search_result_avatar),
            ids!(search_result_skeleton),
            ids!(search_result_progress),
            ids!(search_result_combobox),
            ids!(search_result_date_picker),
            ids!(search_result_menubar),
            ids!(search_result_navigation_menu),
            ids!(search_result_pagination),
            ids!(search_result_resizable),
            ids!(search_result_tabs),
            ids!(search_result_time_picker),
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

        // Helper to hide all pages
        let hide_all = |cx: &mut Cx, ui: &WidgetRef| {
            ui.view(ids!(getting_started_page)).set_visible(cx, false);
            ui.view(ids!(button_page)).set_visible(cx, false);
            ui.view(ids!(input_page)).set_visible(cx, false);
            ui.view(ids!(label_page)).set_visible(cx, false);
            ui.view(ids!(badge_page)).set_visible(cx, false);
            ui.view(ids!(separator_page)).set_visible(cx, false);
            ui.view(ids!(radio_group_page)).set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page)).set_visible(cx, false);
            ui.view(ids!(scroll_area_page)).set_visible(cx, false);
            ui.view(ids!(card_page)).set_visible(cx, false);
            ui.view(ids!(checkbox_page)).set_visible(cx, false);
            ui.view(ids!(switch_page)).set_visible(cx, false);
            ui.view(ids!(slider_page)).set_visible(cx, false);
            ui.view(ids!(avatar_page)).set_visible(cx, false);
            ui.view(ids!(skeleton_page)).set_visible(cx, false);
            ui.view(ids!(progress_page)).set_visible(cx, false);
            ui.view(ids!(combobox_page)).set_visible(cx, false);
            ui.view(ids!(date_picker_page)).set_visible(cx, false);
            ui.view(ids!(menubar_page)).set_visible(cx, false);
            ui.view(ids!(navigation_menu_page)).set_visible(cx, false);
            ui.view(ids!(pagination_page)).set_visible(cx, false);
            ui.view(ids!(resizable_page)).set_visible(cx, false);
            ui.view(ids!(tabs_page)).set_visible(cx, false);
            ui.view(ids!(time_picker_page)).set_visible(cx, false);

            ui.view(ids!(getting_started_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(button_page_mobile)).set_visible(cx, false);
            ui.view(ids!(input_page_mobile)).set_visible(cx, false);
            ui.view(ids!(label_page_mobile)).set_visible(cx, false);
            ui.view(ids!(badge_page_mobile)).set_visible(cx, false);
            ui.view(ids!(separator_page_mobile)).set_visible(cx, false);
            ui.view(ids!(radio_group_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(dropdown_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(scroll_area_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(card_page_mobile)).set_visible(cx, false);
            ui.view(ids!(checkbox_page_mobile)).set_visible(cx, false);
            ui.view(ids!(switch_page_mobile)).set_visible(cx, false);
            ui.view(ids!(slider_page_mobile)).set_visible(cx, false);
            ui.view(ids!(avatar_page_mobile)).set_visible(cx, false);
            ui.view(ids!(skeleton_page_mobile)).set_visible(cx, false);
            ui.view(ids!(progress_page_mobile)).set_visible(cx, false);
            ui.view(ids!(combobox_page_mobile)).set_visible(cx, false);
            ui.view(ids!(date_picker_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(menubar_page_mobile)).set_visible(cx, false);
            ui.view(ids!(navigation_menu_page_mobile))
                .set_visible(cx, false);
            ui.view(ids!(pagination_page_mobile)).set_visible(cx, false);
            ui.view(ids!(resizable_page_mobile)).set_visible(cx, false);
            ui.view(ids!(tabs_page_mobile)).set_visible(cx, false);
            ui.view(ids!(time_picker_page_mobile))
                .set_visible(cx, false);
        };

        hide_all(cx, &ui);

        // Navigate based on component name
        match component {
            "button" => {
                ui.view(ids!(button_page)).set_visible(cx, true);
                ui.view(ids!(button_page_mobile)).set_visible(cx, true);
            }
            "input" => {
                ui.view(ids!(input_page)).set_visible(cx, true);
                ui.view(ids!(input_page_mobile)).set_visible(cx, true);
            }
            "label" => {
                ui.view(ids!(label_page)).set_visible(cx, true);
                ui.view(ids!(label_page_mobile)).set_visible(cx, true);
            }
            "badge" => {
                ui.view(ids!(badge_page)).set_visible(cx, true);
                ui.view(ids!(badge_page_mobile)).set_visible(cx, true);
            }
            "separator" => {
                ui.view(ids!(separator_page)).set_visible(cx, true);
                ui.view(ids!(separator_page_mobile)).set_visible(cx, true);
            }
            "radio_group" => {
                ui.view(ids!(radio_group_page)).set_visible(cx, true);
                ui.view(ids!(radio_group_page_mobile)).set_visible(cx, true);
            }
            "dropdown_menu" => {
                ui.view(ids!(dropdown_menu_page)).set_visible(cx, true);
                ui.view(ids!(dropdown_menu_page_mobile))
                    .set_visible(cx, true);
            }
            "scroll_area" => {
                ui.view(ids!(scroll_area_page)).set_visible(cx, true);
                ui.view(ids!(scroll_area_page_mobile)).set_visible(cx, true);
            }
            "card" => {
                ui.view(ids!(card_page)).set_visible(cx, true);
                ui.view(ids!(card_page_mobile)).set_visible(cx, true);
            }
            "checkbox" => {
                ui.view(ids!(checkbox_page)).set_visible(cx, true);
                ui.view(ids!(checkbox_page_mobile)).set_visible(cx, true);
            }
            "switch" => {
                ui.view(ids!(switch_page)).set_visible(cx, true);
                ui.view(ids!(switch_page_mobile)).set_visible(cx, true);
            }
            "slider" => {
                ui.view(ids!(slider_page)).set_visible(cx, true);
                ui.view(ids!(slider_page_mobile)).set_visible(cx, true);
            }
            "avatar" => {
                ui.view(ids!(avatar_page)).set_visible(cx, true);
                ui.view(ids!(avatar_page_mobile)).set_visible(cx, true);
            }
            "skeleton" => {
                ui.view(ids!(skeleton_page)).set_visible(cx, true);
                ui.view(ids!(skeleton_page_mobile)).set_visible(cx, true);
            }
            "progress" => {
                ui.view(ids!(progress_page)).set_visible(cx, true);
                ui.view(ids!(progress_page_mobile)).set_visible(cx, true);
            }
            "combobox" => {
                ui.view(ids!(combobox_page)).set_visible(cx, true);
                ui.view(ids!(combobox_page_mobile)).set_visible(cx, true);
            }
            "date_picker" => {
                ui.view(ids!(date_picker_page)).set_visible(cx, true);
                ui.view(ids!(date_picker_page_mobile)).set_visible(cx, true);
            }
            "menubar" => {
                ui.view(ids!(menubar_page)).set_visible(cx, true);
                ui.view(ids!(menubar_page_mobile)).set_visible(cx, true);
            }
            "navigation_menu" => {
                ui.view(ids!(navigation_menu_page)).set_visible(cx, true);
                ui.view(ids!(navigation_menu_page_mobile))
                    .set_visible(cx, true);
            }
            "pagination" => {
                ui.view(ids!(pagination_page)).set_visible(cx, true);
                ui.view(ids!(pagination_page_mobile)).set_visible(cx, true);
            }
            "resizable" => {
                ui.view(ids!(resizable_page)).set_visible(cx, true);
                ui.view(ids!(resizable_page_mobile)).set_visible(cx, true);
            }
            "tabs" => {
                ui.view(ids!(tabs_page)).set_visible(cx, true);
                ui.view(ids!(tabs_page_mobile)).set_visible(cx, true);
            }
            "time_picker" => {
                ui.view(ids!(time_picker_page)).set_visible(cx, true);
                ui.view(ids!(time_picker_page_mobile)).set_visible(cx, true);
            }
            _ => {
                ui.view(ids!(getting_started_page)).set_visible(cx, true);
                ui.view(ids!(getting_started_page_mobile))
                    .set_visible(cx, true);
            }
        }

        // Update sidebar active states
        let sidebar_items = [
            ("button", ids!(component_link_button)),
            ("input", ids!(component_link_input)),
            ("label", ids!(component_link_label)),
            ("badge", ids!(component_link_badge)),
            ("separator", ids!(component_link_separator)),
            ("radio_group", ids!(component_link_radio_group)),
            ("dropdown_menu", ids!(component_link_dropdown_menu)),
            ("scroll_area", ids!(component_link_scroll_area)),
            ("card", ids!(component_link_card)),
            ("checkbox", ids!(component_link_checkbox)),
            ("switch", ids!(component_link_switch)),
            ("slider", ids!(component_link_slider)),
            ("avatar", ids!(component_link_avatar)),
            ("skeleton", ids!(component_link_skeleton)),
            ("progress", ids!(component_link_progress)),
            ("combobox", ids!(component_link_combobox)),
            ("date_picker", ids!(component_link_date_picker)),
            ("menubar", ids!(component_link_menubar)),
            ("navigation_menu", ids!(component_link_navigation_menu)),
            ("pagination", ids!(component_link_pagination)),
            ("resizable", ids!(component_link_resizable)),
            ("tabs", ids!(component_link_tabs)),
            ("time_picker", ids!(component_link_time_picker)),
        ];

        for (name, id) in sidebar_items.iter() {
            if *name == component {
                // In a real implementation we might want to set a "selected" state on the button
                // But for now we just ensure the button is visible (it always is)
                // ui.button(*id).set_selected(cx, true);
            } else {
                // ui.button(*id).set_selected(cx, false);
            }
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
