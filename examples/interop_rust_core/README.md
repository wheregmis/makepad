# makepad-example-interop-rust-core

Focused reference for Rust-core, Splash-driven interop.

## Contract
- Rust owns canonical state (`counter`, `step`, `status`, `events`).
- Splash owns UI composition and emits intents.
- Bridge methods connect Splash intents to Rust mutations.
- Rust triggers Splash render refresh after mutations.

## Mapping
| Flow | Path |
|---|---|
| Splash intent | `on_click` -> `ui.bridge.increment()` / `decrement()` / `set_step()` / `reset()` |
| Rust mutation | `Bridge::script_call` -> mutate shared `AppState` |
| Render refresh | `script_eval!(cx, { ui.main_view.render() })` |
| Rust action path | `App::handle_actions` (`rust_bump_btn`) -> mutate state -> render |
| Read model | `main_view.on_render` -> `ui.bridge.get_*()` |

## Key Snippets

### Bridge method dispatch (`script_call`)
```rust
if method == live_id!(increment) {
    let next = increment(&shared, "splash.increment");
    self.notify_and_render(vm, next);
    return ScriptAsyncResult::Return((next as f64).into());
}
```

### Splash render reads Rust-owned state
```text
on_render: || {
    let counter = ui.bridge.get_counter()
    let step = ui.bridge.get_step()
    let status = ui.bridge.get_status()
    let events = ui.bridge.get_events()
    // build UI from getters
}
```

### Rust-side action updates state + renders
```rust
if self.ui.button(cx, ids!(rust_bump_btn)).clicked(actions) {
    bridge::increment_by(&self.state, 5, "rust.button:+5");
    script_eval!(cx, { ui.main_view.render() });
}
```

## Run
```bash
cargo run -p makepad-example-interop-rust-core --release
```

## When To Use This Model
- Use this model when business/domain logic must stay in Rust and Splash is the dynamic UI layer.
- Prefer script-first when domain logic is lightweight and lives comfortably in Splash.
