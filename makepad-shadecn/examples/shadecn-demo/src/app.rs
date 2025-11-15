use makepad_widgets::*;
use makepad_shadecn_core::*;
use makepad_shadecn_button::*;
use makepad_shadecn_input::*;
use makepad_shadecn_card::*;
use makepad_shadecn_checkbox::*;
use makepad_shadecn_toggle::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;


    AppWindow = {{AppWindow}} {
        ui: <Root> {
            main_window = <Window> {
                window: {
                    inner_size: vec2(1100.0, 780.0),
                    position: vec2(140.0, 120.0),
                    title: "Shadecn UI Components"
                }

                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    show_bg: true,
                    draw_bg: {
                        fn pixel(self) -> vec4 {
                            let gradient = mix(#f8fafc, #e2e8f0, self.pos.y);
                            return gradient;
                        }
                    }

                    scroller = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Down,
                        spacing: 32,
                        padding: {left: 40, top: 36, right: 40, bottom: 48},
                        scroll_bars: <ScrollBars> {
                            show_scroll_x: false,
                            show_scroll_y: true,
                        }

                        content_view = <View> {
                            width: Fill,
                            height: Fit,
                            flow: Down,
                            spacing: 32,

                            header_bar = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Right,
                                spacing: 12,
                                align: {x: 1.0, y: 0.0},
                                margin: {left: 0, top: 0, right: 0, bottom: 16},

                                theme_label = <Label> {
                                    text: "Dark mode",
                                    draw_text: {
                                        text_style: {
                                            font_size: 14.0,
                                        }
                                        color: #475569
                                    }
                                }

                                theme_toggle = <ShadecnToggle> {
                                    width: Fit,
                                    height: Fit,
                                }
                            }

                            welcome = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Down,
                                spacing: 8,

                                title = <Label> {
                                    text: "Shadecn Components",
                                    draw_text: {
                                        text_style: {
                                            font_size: 30.0,
                                        }
                                        wrap: Word,
                                        color: #0f172a
                                    }
                                }

                                subtitle = <Label> {
                                    text: "Composable UI primitives inspired by shadecn/ui, rebuilt with Makepad widgets and design tokens.",
                                    draw_text: {
                                        text_style: {
                                            font_size: 15.0,
                                        }
                                        wrap: Word,
                                        color: #475569
                                    }
                                }
                            }

                            action_row = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Right { wrap: true },
                                spacing: 12,

                                get_started = <ShadecnButton> {
                                    text: "Get started"
                                }

                                docs_link = <ShadecnButtonGhost> {
                                    text: "View documentation"
                                }
                            }

                            component_stack = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Down,
                                spacing: 24,

                                button_section = <ShadecnCard> {
                                    ShadecnCardHeader = <ShadecnCardHeader> {
                                        ShadecnCardTitle = <ShadecnCardTitle> {
                                            text: "Button Variants"
                                        }
                                        ShadecnCardDescription = <ShadecnCardDescription> {
                                            text: "Primary actions, subtle options, and stateful buttons."
                                        }
                                    }

                                    ShadecnCardContent = <ShadecnCardContent> {
                                        width: Fill,
                                        flow: Right { wrap: true },
                                        spacing: 12,

                                        <ShadecnButton> { text: "Primary" }
                                        <ShadecnButtonSecondary> { text: "Secondary" }
                                        <ShadecnButtonOutline> { text: "Outline" }
                                        <ShadecnButtonDestructive> { text: "Destructive" }
                                        <ShadecnButtonGhost> { text: "Ghost" }
                                    }
                                }

                                form_section = <ShadecnCard> {
                                    ShadecnCardHeader = <ShadecnCardHeader> {
                                        ShadecnCardTitle = <ShadecnCardTitle> {
                                            text: "Quick Form"
                                        }
                                        ShadecnCardDescription = <ShadecnCardDescription> {
                                            text: "Inputs, validation, and agreement with shadecn styling."
                                        }
                                    }

                                    ShadecnCardContent = <ShadecnCardContent> {
                                        width: Fill,
                                        flow: Down,
                                        spacing: 18,

                                        name_group = <View> {
                                            width: Fill,
                                            flow: Down,
                                            spacing: 6,

                                            label = <Label> {
                                                text: "Full name",
                                                draw_text: {
                                                    text_style: {
                                                        font_size: 14.0,
                                                    }
                                                    color: #0f172a
                                                }
                                            }

                                            input = <ShadecnInput> {
                                                empty_text: "Jane Appleseed"
                                            }
                                        }

                                        email_group = <View> {
                                            width: Fill,
                                            flow: Down,
                                            spacing: 6,

                                            label = <Label> {
                                                text: "Work email",
                                                draw_text: {
                                                    text_style: {
                                                        font_size: 14.0,
                                                    }
                                                    color: #0f172a
                                                }
                                            }

                                            email_input = <ShadecnInput> {
                                                empty_text: "you@company.com"
                                            }
                                        }

                                        checkbox_group = <View> {
                                            width: Fill,
                                            flow: Down,
                                            spacing: 10,

                                            agree_checkbox = <ShadecnCheckbox> {
                                                text: "I agree to the terms"
                                                active: true
                                            }

                                            marketing_checkbox = <ShadecnCheckbox> {
                                                text: "Receive occasional product updates"
                                            }
                                        }
                                    }

                                    ShadecnCardFooter = <ShadecnCardFooter> {
                                        footer_hint = <Label> {
                                            text: "We care about your privacy. Unsubscribe at any time."
                                            draw_text: {
                                                text_style: {
                                                    font_size: 12.0,
                                                }
                                                wrap: Word,
                                                color: #94a3b8
                                            }
                                        }

                                        submit = <ShadecnButton> {
                                            text: "Create account"
                                        }
                                    }
                                }

                                preferences_section = <ShadecnCard> {
                                    ShadecnCardHeader = <ShadecnCardHeader> {
                                        ShadecnCardTitle = <ShadecnCardTitle> {
                                            text: "Preferences"
                                        }
                                        ShadecnCardDescription = <ShadecnCardDescription> {
                                            text: "Toggle personalized experiences for your workspace."
                                        }
                                    }

                                    ShadecnCardContent = <ShadecnCardContent> {
                                        width: Fill,
                                        flow: Down,
                                        spacing: 14,

                                        <ShadecnCheckbox> {
                                            text: "Enable dark mode"
                                            active: true
                                        }

                                        <ShadecnCheckbox> {
                                            text: "Product research participation"
                                        }

                                        <ShadecnCheckbox> {
                                            text: "Weekly summary email"
                                        }
                                    }

                                    ShadecnCardFooter = <ShadecnCardFooter> {
                                        <ShadecnButtonSecondary> {
                                            text: "Save preferences"
                                        }
                                    }
                                }
                            }

                            newsletter_section = <ShadecnCard> {
                                ShadecnCardHeader = <ShadecnCardHeader> {
                                    ShadecnCardTitle = <ShadecnCardTitle> {
                                        text: "Editorial"
                                    }
                                    ShadecnCardDescription = <ShadecnCardDescription> {
                                        text: "Highlight curated stories using cards and supporting text."
                                    }
                                }

                                ShadecnCardContent = <ShadecnCardContent> {
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10,

                                    intro = <Label> {
                                        text: "Shadecn UI components bring a consistent visual language to Makepad. Every piece is powered by the same design tokens for typography, color, and spacing."
                                        draw_text: {
                                            text_style: {
                                                font_size: 14.0,
                                            }
                                            wrap: Word,
                                            color: #475569
                                        }
                                    }
                                }
                            }

                            about_section = <ShadecnCard> {
                                ShadecnCardHeader = <ShadecnCardHeader> {
                                    ShadecnCardTitle = <ShadecnCardTitle> {
                                        text: "About Shadecn Components"
                                    }
                                    ShadecnCardDescription = <ShadecnCardDescription> {
                                        text: "Principles that guide composable, theme-aware UI."
                                    }
                                }

                                ShadecnCardContent = <ShadecnCardContent> {
                                    width: Fill,
                                    flow: Down,
                                    spacing: 10,

                                    principle = <Label> {
                                        text: "Each component is type-safe, themed with shared tokens, and intentionally minimal so you can build your own visual system on top. These demos pair buttons, inputs, and cards exactly the way the ui-zoo reference encourages—small polished primitives composed into real layouts."
                                        draw_text: {
                                            text_style: {
                                                font_size: 14.0,
                                            }
                                            wrap: Word,
                                            color: #334155
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
    is_dark_mode: bool,
}

impl LiveRegister for AppWindow {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        makepad_shadecn_core::live_design(cx);
        makepad_shadecn_button::live_design(cx);
        makepad_shadecn_input::live_design(cx);
        makepad_shadecn_card::live_design(cx);
        makepad_shadecn_checkbox::live_design(cx);
        makepad_shadecn_toggle::live_design(cx);
        
        // Note: Keep default Makepad theme for widgets to work properly
        // The shadecn themes are design tokens, not full theme replacements
    }
}

impl LiveHook for AppWindow {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.is_dark_mode = false;
        // Set initial theme state to match toggle
        self.ui.check_box(ids!(theme_toggle)).set_active(cx, false);
    }
}

impl MatchEvent for AppWindow {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle theme toggle
        if let Some(is_active) = self.ui.check_box(ids!(theme_toggle)).changed(actions) {
            if is_active != self.is_dark_mode {
                self.is_dark_mode = is_active;
                // Switch theme using standard Makepad themes
                if self.is_dark_mode {
                    cx.link(live_id!(theme), live_id!(theme_desktop_dark));
                } else {
                    cx.link(live_id!(theme), live_id!(theme_desktop_light));
                }
                cx.reload_ui_dsl();
                cx.redraw_all();
            }
        }
    }
}

impl AppMain for AppWindow {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
