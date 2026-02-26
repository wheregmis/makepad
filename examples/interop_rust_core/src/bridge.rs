use crate::makepad_widgets::*;
use crate::makepad_widgets::makepad_script::ScriptFnRef;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub counter: i64,
    pub step: i64,
    pub status: String,
    pub events: Vec<String>,
}

pub type SharedState = Arc<Mutex<AppState>>;

static SHARED_STATE: OnceLock<SharedState> = OnceLock::new();

fn lock_state<'a>(state: &'a SharedState) -> MutexGuard<'a, AppState> {
    state.lock().unwrap_or_else(|err| err.into_inner())
}

fn push_event(state: &mut AppState, message: String) {
    let seq = state.events.len() + 1;
    state.events.push(format!("{seq:03}: {message}"));
}

fn apply_counter_delta(state: &mut AppState, delta: i64, origin: &str) -> i64 {
    state.counter += delta;
    state.status = format!("{origin} counter={} step={}", state.counter, state.step);
    push_event(state, format!("{origin} delta={delta}"));
    state.counter
}

pub fn install_shared_state(state: SharedState) {
    let _ = SHARED_STATE.set(state);
}

pub fn shared_state() -> SharedState {
    SHARED_STATE
        .get_or_init(|| Arc::new(Mutex::new(AppState::default())))
        .clone()
}

pub fn reset_state(state: &SharedState) {
    let mut state = lock_state(state);
    state.counter = 0;
    state.step = 1;
    state.status = "startup reset".to_string();
    state.events.clear();
    push_event(&mut state, "startup reset".to_string());
}

pub fn snapshot(state: &SharedState) -> AppState {
    lock_state(state).clone()
}

pub fn increment(state: &SharedState, origin: &str) -> i64 {
    let mut state = lock_state(state);
    let step = state.step;
    apply_counter_delta(&mut state, step, origin)
}

pub fn decrement(state: &SharedState, origin: &str) -> i64 {
    let mut state = lock_state(state);
    let step = state.step;
    apply_counter_delta(&mut state, -step, origin)
}

pub fn increment_by(state: &SharedState, delta: i64, origin: &str) -> i64 {
    let mut state = lock_state(state);
    apply_counter_delta(&mut state, delta, origin)
}

pub fn set_step(state: &SharedState, raw: f64, origin: &str) -> bool {
    if !raw.is_finite() {
        return false;
    }
    let rounded = raw.round();
    if rounded < 1.0 {
        return false;
    }
    let next_step = rounded as i64;
    let mut state = lock_state(state);
    if state.step == next_step {
        return false;
    }
    state.step = next_step;
    let step = state.step;
    state.status = format!("{origin} step={step}");
    push_event(&mut state, format!("{origin} step={step}"));
    true
}

pub fn reset_counter(state: &SharedState, origin: &str) {
    let mut state = lock_state(state);
    state.counter = 0;
    state.status = format!("{origin} counter=0 step={}", state.step);
    push_event(&mut state, format!("{origin}"));
}

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.BridgeBase = #(Bridge::register_widget(vm))

    mod.widgets.Bridge = set_type_default() do mod.widgets.BridgeBase {
        width: 0
        height: 0
        visible: false
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct Bridge {
    #[source]
    source: ScriptObjectRef,
    #[deref]
    view: View,
    #[live]
    on_state_changed: ScriptFnRef,
}

impl Bridge {
    fn first_arg(vm: &mut ScriptVm, args: ScriptValue) -> Option<ScriptValue> {
        let args_obj = args.as_object()?;
        let trap = vm.bx.threads.cur().trap.pass();
        Some(vm.bx.heap.vec_value(args_obj, 0, trap))
    }

    fn first_arg_f64(vm: &mut ScriptVm, args: ScriptValue) -> Option<f64> {
        let value = Self::first_arg(vm, args)?;
        if let Some(number) = value.as_f64() {
            return Some(number);
        }
        if value.is_string_like() {
            return vm
                .string_with(value, |_vm, text| text.parse::<f64>().ok())
                .flatten();
        }
        None
    }

    fn events_array_value(vm: &mut ScriptVm, events: &[String]) -> ScriptValue {
        let array = vm.bx.heap.new_array();
        let trap = vm.bx.threads.cur().trap.pass();
        for entry in events {
            let value = vm.bx.heap.new_string_from_str(entry);
            vm.bx.heap.array_push(array, value, trap);
        }
        array.into()
    }

    fn notify_and_render(&mut self, vm: &mut ScriptVm, counter: i64) {
        let uid = self.widget_uid();
        let source = self.source.clone();
        let callback = self.on_state_changed.clone();
        vm.with_cx_mut(|cx| {
            cx.widget_to_script_call(uid, NIL, source, callback, &[(counter as f64).into()]);
            script_eval!(cx, {
                ui.main_view.render()
            });
        });
    }
}

impl Widget for Bridge {
    fn script_call(
        &mut self,
        vm: &mut ScriptVm,
        method: LiveId,
        args: ScriptValue,
    ) -> ScriptAsyncResult {
        let shared = shared_state();

        if method == live_id!(get_counter) {
            let snapshot = snapshot(&shared);
            return ScriptAsyncResult::Return((snapshot.counter as f64).into());
        }

        if method == live_id!(get_step) {
            let snapshot = snapshot(&shared);
            return ScriptAsyncResult::Return((snapshot.step as f64).into());
        }

        if method == live_id!(get_status) {
            let snapshot = snapshot(&shared);
            let value = vm.bx.heap.new_string_from_str(&snapshot.status);
            return ScriptAsyncResult::Return(value);
        }

        if method == live_id!(get_events) {
            let snapshot = snapshot(&shared);
            return ScriptAsyncResult::Return(Self::events_array_value(vm, &snapshot.events));
        }

        if method == live_id!(increment) {
            let next = increment(&shared, "splash.increment");
            self.notify_and_render(vm, next);
            return ScriptAsyncResult::Return((next as f64).into());
        }

        if method == live_id!(decrement) {
            let next = decrement(&shared, "splash.decrement");
            self.notify_and_render(vm, next);
            return ScriptAsyncResult::Return((next as f64).into());
        }

        if method == live_id!(set_step) {
            if let Some(raw) = Self::first_arg_f64(vm, args) {
                if set_step(&shared, raw, "splash.set_step") {
                    let snapshot = snapshot(&shared);
                    self.notify_and_render(vm, snapshot.counter);
                }
            }
            return ScriptAsyncResult::Return(NIL);
        }

        if method == live_id!(reset) {
            reset_counter(&shared, "splash.reset");
            let snapshot = snapshot(&shared);
            self.notify_and_render(vm, snapshot.counter);
            return ScriptAsyncResult::Return(NIL);
        }

        ScriptAsyncResult::MethodNotFound
    }
}
