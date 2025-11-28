# Native Components Example

This example demonstrates embedding native platform UI components in Makepad applications.

## Overview

The `NativeView` widget allows you to embed native platform UI components (UIKit on iOS, Android Views on Android) into your Makepad application. The native components render to textures which are then composited into Makepad's GPU rendering pipeline.

## Supported Components

- **Button** - Native platform button
- **TextField** - Native text input field
- **Label** - Native text label
- **Switch** - Native toggle switch
- **Slider** - Native slider control
- **ProgressIndicator** - Native progress bar
- **Custom** - Custom native views (platform-specific)

## Usage

```rust
live_design! {
    // Native Button
    my_button = <NativeView> {
        width: 200,
        height: 50,
        view_kind: Button,
        label: "Click Me!"
    }
    
    // Native TextField
    my_text_field = <NativeView> {
        width: 300,
        height: 44,
        view_kind: TextField,
        placeholder: "Enter text..."
        text: ""
    }
    
    // Native Switch
    my_switch = <NativeView> {
        width: 60,
        height: 32,
        view_kind: Switch,
        switch_on: false
    }
    
    // Native Slider
    my_slider = <NativeView> {
        width: 250,
        height: 32,
        view_kind: Slider,
        slider_value: 0.5,
        slider_min: 0.0,
        slider_max: 1.0
    }
}
```

## Handling Events

```rust
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Button clicks
        if self.ui.native_view(id!(my_button)).clicked(actions) {
            log!("Button clicked!");
        }
        
        // Text changes
        if let Some(text) = self.ui.native_view(id!(my_text_field)).text_changed(actions) {
            log!("Text changed: {}", text);
        }
        
        // Switch changes
        if let Some(on) = self.ui.native_view(id!(my_switch)).switch_changed(actions) {
            log!("Switch is now: {}", if on { "ON" } else { "OFF" });
        }
        
        // Slider changes
        if let Some(value) = self.ui.native_view(id!(my_slider)).slider_changed(actions) {
            log!("Slider value: {}", value);
        }
    }
}
```

## Running

```bash
# Run on desktop (native components will be simulated)
cargo run -p makepad-example-native-components

# Run on iOS
cargo makepad ios run-sim -p makepad-example-native-components

# Run on Android
cargo makepad android run -p makepad-example-native-components
```

## Architecture

The native component embedding system works as follows:

1. **Platform Layer**: Native views (UIView on iOS, View on Android) are created and managed
2. **Texture Sharing**: Native views render to off-screen surfaces (CAMetalLayer on iOS, SurfaceTexture on Android)
3. **Compositing**: The textures are imported into Makepad's rendering pipeline
4. **Event Routing**: Touch events are forwarded to native views, and native events are routed back to Makepad

This approach allows native components to integrate seamlessly with Makepad's layout system while maintaining native look and feel.

## Limitations

- Native components are rendered to textures, which may have a slight performance overhead
- Some native component features may not be fully supported
- Platform-specific styling differences exist between iOS and Android

