pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*
    let state = {
        counter: 0
        is_light_theme: true
    }
    mod.state = state
    mod.theme = mod.themes.light
    startup() do #(App::script_component(vm)){
        ui: Root{
            on_startup:||{ // right now render isnt called automatically yet
                ui.main_view.render()
            }
            main_window := Window{
                window.inner_size: vec2(420, 220)
                body +: {
                    main_view := View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 12
                        align: Center
                        on_render: ||{
                            counter_label := Label{
                                text: "Count: " + state.counter
                                draw_text.text_style.font_size: 24
                                draw_text.color: if state.is_light_theme #x222222 else #xfff
                            }
                            theme_label := Label{
                                text: if state.is_light_theme "Theme: Light" else "Theme: Dark"
                                draw_text.text_style.font_size: 16
                                draw_text.color: if state.is_light_theme #x444444 else #xdddddd
                            }
                        }
                    }
                    theme_button := Button{
                        text: "Switch Theme"
                    }
                    increment_button := Button{
                        text: "Increment"
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(theme_button)).clicked(actions) {
            script_eval!(cx,{
                if mod.state.is_light_theme{
                    mod.state.is_light_theme = false
                    mod.theme = mod.themes.dark
                }
                else{
                    mod.state.is_light_theme = true
                    mod.theme = mod.themes.light
                }
                ui.main_view.render()
            });
        }
        if self.ui.button(cx, ids!(increment_button)).clicked(actions) {
            script_eval!(cx,{
                mod.state.counter += 1
                ui.main_view.render()
            });
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
