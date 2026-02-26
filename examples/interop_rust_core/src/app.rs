use crate::bridge::{self, AppState, SharedState};
use makepad_widgets::*;
use std::sync::{Arc, Mutex};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(App::script_component(vm)){
        ui: Root{
            on_startup: || {
                ui.main_view.render()
            }
            main_window := Window{
                window.title: "Rust Core + Splash UI Interop"
                window.inner_size: vec2(940, 760)
                body +: {
                    flow: Down
                    spacing: 10
                    padding: 14

                    bridge := Bridge{
                        on_state_changed: |next_counter| {
                            ui.callback_echo.set_text("Bridge callback counter=" + next_counter)
                        }
                    }

                    main_view := View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 10
                        on_render: || {
                            let counter = ui.bridge.get_counter()
                            let step = ui.bridge.get_step()
                            let status = ui.bridge.get_status()
                            let events = ui.bridge.get_events()
                            let event_count = events.len()
                            let first_event = if event_count > 12 event_count - 12 else 0

                            RoundedView{
                                width: Fill
                                height: Fit
                                new_batch: true
                                flow: Down
                                spacing: 6
                                padding: 14
                                draw_bg.color: theme.color_bg_container
                                draw_bg.border_radius: 8.0

                                Label{
                                    text: "Rust-owned state, Splash-driven UI"
                                    draw_text.color: theme.color_label_inner
                                    draw_text.text_style: theme.font_bold{font_size: theme.font_size_2}
                                }
                                Label{
                                    text: "Counter: " + counter + " | Step: " + step
                                    draw_text.color: theme.color_label_inner
                                }
                                Label{
                                    text: "Status: " + status
                                    draw_text.color: theme.color_label_inner_inactive
                                }
                            }

                            RoundedView{
                                width: Fill
                                height: Fit
                                new_batch: true
                                flow: Down
                                spacing: 8
                                padding: 14
                                draw_bg.color: theme.color_bg_container
                                draw_bg.border_radius: 8.0

                                Label{
                                    text: "Splash intent controls (script -> Rust bridge)"
                                    draw_text.color: theme.color_label_inner
                                    draw_text.text_style: theme.font_bold{font_size: theme.font_size_3}
                                }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8
                                    Button{
                                        text: "- step"
                                        on_click: || ui.bridge.decrement()
                                    }
                                    Button{
                                        text: "+ step"
                                        on_click: || ui.bridge.increment()
                                    }
                                    ButtonFlat{
                                        text: "Reset"
                                        on_click: || ui.bridge.reset()
                                    }
                                }

                                View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8
                                    ButtonFlat{
                                        text: "Step -1"
                                        on_click: || ui.bridge.set_step(ui.bridge.get_step() - 1)
                                    }
                                    ButtonFlat{
                                        text: "Step +1"
                                        on_click: || ui.bridge.set_step(ui.bridge.get_step() + 1)
                                    }
                                    ButtonFlat{
                                        text: "Invalid step"
                                        on_click: || ui.bridge.set_step("bad-value")
                                    }
                                }

                                Label{
                                    text: "Invalid step is ignored safely (no panic, step unchanged)."
                                    draw_text.color: theme.color_label_inner_inactive
                                    draw_text.text_style.font_size: theme.font_size_code
                                }
                            }

                            RoundedView{
                                width: Fill
                                height: Fit
                                new_batch: true
                                flow: Down
                                spacing: 6
                                padding: 14
                                draw_bg.color: theme.color_bg_container
                                draw_bg.border_radius: 8.0

                                Label{
                                    text: "State transition log (from Rust)"
                                    draw_text.color: theme.color_label_inner
                                    draw_text.text_style: theme.font_bold{font_size: theme.font_size_3}
                                }
                                if event_count == 0 {
                                    Label{
                                        text: "No events yet"
                                        draw_text.color: theme.color_label_inner_inactive
                                    }
                                }
                                else for i in first_event..event_count {
                                    Label{
                                        text: events[i]
                                        draw_text.color: theme.color_label_inner
                                        draw_text.text_style.font_size: theme.font_size_code
                                    }
                                }
                            }
                        }
                    }

                    RoundedView{
                        width: Fill
                        height: Fit
                        new_batch: true
                        flow: Down
                        spacing: 8
                        padding: 14
                        draw_bg.color: theme.color_bg_container
                        draw_bg.border_radius: 8.0

                        Label{
                            text: "Rust action path"
                            draw_text.color: theme.color_label_inner
                            draw_text.text_style: theme.font_bold{font_size: theme.font_size_3}
                        }

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            rust_bump_btn := Button{
                                text: "+5 (Rust handle_actions)"
                            }
                            callback_echo := TextInput{
                                width: Fill
                                height: Fit
                                is_read_only: true
                                empty_text: "Bridge callback output"
                            }
                        }
                    }
                }
            }
        }
    }
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        let state: SharedState = Arc::new(Mutex::new(AppState::default()));
        bridge::install_shared_state(state.clone());

        crate::makepad_widgets::script_mod(vm);
        crate::bridge::script_mod(vm);

        let mut app = App::from_script_mod(vm, self::script_mod);
        app.state = state;
        app
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: SharedState,
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        bridge::reset_state(&self.state);
        self.ui
            .text_input(cx, ids!(callback_echo))
            .set_text(cx, "Bridge callback pending...");
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(rust_bump_btn)).clicked(actions) {
            let next = bridge::increment_by(&self.state, 5, "rust.button:+5");
            self.ui
                .text_input(cx, ids!(callback_echo))
                .set_text(cx, &format!("Rust path counter={next}"));
            script_eval!(cx, {
                ui.main_view.render()
            });
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
