use makepad_shadecn_button::*;
use makepad_shadecn_core::*;
use makepad_shadecn_input::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // Documentation Header
    pub DocHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: (SPACE_4),
        padding: {left: (SPACE_6), top: (SPACE_5), right: (SPACE_6), bottom: (SPACE_5)},
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG_PRIMARY);
            }
        }

        logo = <Label> {
            text: "Shadecn UI",
            draw_text: {
                text_style: {
                    font_size: (FONT_2XL),
                }
                color: (COLOR_FG_PRIMARY)
            }
        }
    }

    // Sidebar Navigation
    pub DocSidebar = <View> {
        width: 280,
        height: Fill,
        flow: Down,
        spacing: 0,
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG_SECONDARY);
            }
        }
        padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_6)},

        sidebar_scroll = <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: (SPACE_4),
            scroll_bars: <ScrollBars> {
                show_scroll_x: false,
                show_scroll_y: true,
            }

            nav_section = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: (SPACE_3),

                nav_title = <Label> {
                    text: "Getting Started",
                    draw_text: {
                        text_style: {
                            font_size: (FONT_BASE),
                        }
                        color: (COLOR_FG_TERTIARY)
                    }
                }

                nav_link_installation = <ShadecnButtonGhost> {
                    text: "Installation",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                nav_link_theme = <ShadecnButtonGhost> {
                    text: "Theme",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                nav_link_getting_started = <ShadecnButtonGhost> {
                    text: "Overview",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }
            }

            components_section = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: (SPACE_3),

                components_title = <Label> {
                    text: "Components",
                    draw_text: {
                        text_style: {
                            font_size: (FONT_BASE),
                        }
                        color: (COLOR_FG_TERTIARY)
                    }
                }

                component_link_button = <ShadecnButtonGhost> {
                    text: "Button",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_input = <ShadecnButtonGhost> {
                    text: "Input",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_label = <ShadecnButtonGhost> {
                    text: "Label",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_badge = <ShadecnButtonGhost> {
                    text: "Badge",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_separator = <ShadecnButtonGhost> {
                    text: "Separator",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_radio_group = <ShadecnButtonGhost> {
                    text: "RadioGroup",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_dropdown_menu = <ShadecnButtonGhost> {
                    text: "Dropdown Menu",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_scroll_area = <ShadecnButtonGhost> {
                    text: "Scroll Area",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_card = <ShadecnButtonGhost> {
                    text: "Card",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_checkbox = <ShadecnButtonGhost> {
                    text: "Checkbox",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_switch = <ShadecnButtonGhost> {
                    text: "Switch",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_slider = <ShadecnButtonGhost> {
                    text: "Slider",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_avatar = <ShadecnButtonGhost> {
                    text: "Avatar",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_skeleton = <ShadecnButtonGhost> {
                    text: "Skeleton",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_progress = <ShadecnButtonGhost> {
                    text: "Progress",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_dialog = <ShadecnButtonGhost> {
                    text: "Dialog",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_sheet = <ShadecnButtonGhost> {
                    text: "Sheet",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_drawer = <ShadecnButtonGhost> {
                    text: "Drawer",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_popover = <ShadecnButtonGhost> {
                    text: "Popover",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_hover_card = <ShadecnButtonGhost> {
                    text: "Hover Card",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_alert_dialog = <ShadecnButtonGhost> {
                    text: "Alert Dialog",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_tooltip = <ShadecnButtonGhost> {
                    text: "Tooltip",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_toast = <ShadecnButtonGhost> {
                    text: "Toast",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_tabs = <ShadecnButtonGhost> {
                    text: "Tabs",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_table = <ShadecnButtonGhost> {
                    text: "Table",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_calendar = <ShadecnButtonGhost> {
                    text: "Calendar",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_accordion = <ShadecnButtonGhost> {
                    text: "Accordion",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_alert = <ShadecnButtonGhost> {
                    text: "Alert",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_carousel = <ShadecnButtonGhost> {
                    text: "Carousel",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_collapsible = <ShadecnButtonGhost> {
                    text: "Collapsible",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_context_menu = <ShadecnButtonGhost> {
                    text: "Context Menu",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }

                component_link_command = <ShadecnButtonGhost> {
                    text: "Command",
                    width: Fill,
                    align: {x: 0.0, y: 0.5},
                }
            }
        }
    }

    // Main Content Area
    pub DocContent = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 0,

        search_section = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: (SPACE_3),
            padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_4)},
            align: {x: 0.5, y: 0.0},
            show_bg: true,
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_BG_PRIMARY);
                }
            }

            search_wrapper = <View> {
                width: Fit,
                height: Fit,
                flow: Down,
                spacing: 0,

                search_area = <ShadecnInput> {
                    width: 500,
                    height: Fit,
                    empty_text: "Search components...",
                }

                search_results_popup = <View> {
                    width: 500,
                    height: Fit,
                    flow: Down,
                    spacing: 0,
                    visible: false,
                    show_bg: true,
                    draw_bg: {
                        uniform border_size: 1.0,
                        uniform border_radius: (RADIUS_MD),
                        color: (COLOR_BG_PRIMARY),
                        uniform border_color: (COLOR_BORDER_PRIMARY),
                        fn pixel(self) -> vec4 {
                            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                            sdf.box(
                                self.border_size,
                                self.border_size,
                                self.rect_size.x - (self.border_size * 2.0),
                                self.rect_size.y - (self.border_size * 2.0),
                                max(1.0, self.border_radius)
                            );
                            sdf.fill_keep(self.color);
                            if self.border_size > 0.0 {
                                sdf.stroke(self.border_color, self.border_size);
                            }
                            return sdf.result;
                        }
                    }
                    margin: {top: (SPACE_1)},

                    search_result_list = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Down,
                        spacing: 0,

                        search_result_button = <ShadecnButtonGhost> {
                            text: "Button",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_input = <ShadecnButtonGhost> {
                            text: "Input",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_label = <ShadecnButtonGhost> {
                            text: "Label",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_badge = <ShadecnButtonGhost> {
                            text: "Badge",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_separator = <ShadecnButtonGhost> {
                            text: "Separator",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_radio_group = <ShadecnButtonGhost> {
                            text: "RadioGroup",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_dropdown_menu = <ShadecnButtonGhost> {
                            text: "Dropdown Menu",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_scroll_area = <ShadecnButtonGhost> {
                            text: "Scroll Area",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_card = <ShadecnButtonGhost> {
                            text: "Card",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_checkbox = <ShadecnButtonGhost> {
                            text: "Checkbox",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_switch = <ShadecnButtonGhost> {
                            text: "Switch",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_slider = <ShadecnButtonGhost> {
                            text: "Slider",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_avatar = <ShadecnButtonGhost> {
                            text: "Avatar",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_skeleton = <ShadecnButtonGhost> {
                            text: "Skeleton",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }

                        search_result_progress = <ShadecnButtonGhost> {
                            text: "Progress",
                            width: Fill,
                            align: {x: 0.0, y: 0.5},
                        }
                    }
                }
            }
        }

        content_scroll = <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: (SPACE_6),
            padding: {left: (SPACE_6), top: 0, right: (SPACE_6), bottom: (SPACE_6)},
            scroll_bars: <ScrollBars> {
                show_scroll_x: false,
                show_scroll_y: true,
            }

            DocContentInner = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: (SPACE_6),
            }
        }
    }

    // Right Sidebar for TOC (Optional)
    pub DocTOC = <View> {
        width: 240,
        height: Fill,
        flow: Down,
        spacing: (SPACE_4),
        padding: {left: (SPACE_6), top: (SPACE_6), right: (SPACE_6), bottom: (SPACE_6)},
        show_bg: true,
        draw_bg: {
            fn pixel(self) -> vec4 {
                return (COLOR_BG_SECONDARY);
            }
        }

        toc_title = <Label> {
            text: "On this page",
            draw_text: {
                text_style: {
                    font_size: (FONT_BASE),
                }
                color: (COLOR_FG_TERTIARY)
            }
        }

        toc_link = <Label> {
            text: "Overview",
            draw_text: {
                text_style: {
                    font_size: (FONT_SM),
                }
                color: (COLOR_FG_PRIMARY)
            }
        }

        toc_link2 = <Label> {
            text: "Installation",
            draw_text: {
                text_style: {
                    font_size: (FONT_SM),
                }
                color: (COLOR_FG_PRIMARY)
            }
        }
    }

    // Desktop Layout with Sidebar and Content
    pub DocLayoutDesktop = <View> {
        width: Fill,
        height: Fill,
        flow: Right,
        spacing: 0,

        sidebar = <DocSidebar> {}
        content = <DocContent> {}
    }

    // Mobile Layout (stacked)
    pub DocLayoutMobile = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 0,

        header = <DocHeader> {}
        content = <DocContent> {}
    }
}
