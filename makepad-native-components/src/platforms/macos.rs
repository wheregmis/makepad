// Full macOS native view implementation
// This provides the actual functionality for creating and managing native NSViews

#[cfg(target_os = "macos")]
mod macos_impl {
    use {
        std::collections::HashMap,
        makepad_platform::{
            makepad_math::*,
            native_view::{
                NativeViewId,
                NativeViewConfig,
                NativeViewEvent,
                NativeViewTouchEvent,
                NativeViewHandle,
            },
            // Note: Texture is private, full implementation would need access
            makepad_objc_sys::runtime::{nil, ObjcId},
        },
        makepad_platform::os::apple::apple_sys::*,
        makepad_platform::os::apple::apple_util::{str_to_nsstring, nsstring_to_string},
    };

    const NS_CONTROL_STATE_VALUE_ON: i32 = 1;
    const NS_CONTROL_STATE_VALUE_OFF: i32 = 0;

    /// macOS-specific native view state
    pub struct MacosNativeViewState {
        /// The native NSView
        pub view: ObjcId,
        /// The container view used for clipping
        pub container_view: ObjcId,
        /// Off-screen render target (CAMetalLayer)
        pub metal_layer: ObjcId,
        /// Current frame
        pub frame: Rect,
        /// Current clip
        pub clip: Rect,
        /// Texture for rendering (would be Texture type in full implementation)
        pub texture: Option<()>,
        /// Width in pixels
        pub width: usize,
        /// Height in pixels
        pub height: usize,
        /// Scale factor
        pub scale: f64,
        pub view_type: String,
        pub properties: HashMap<String, String>,
    }

    /// Full implementation manager for macOS native views
    pub struct MacosNativeViewManagerImpl {
        views: HashMap<NativeViewId, MacosNativeViewState>,
        pending_events: Vec<NativeViewEvent>,
        content_view: ObjcId,
        dpi_factor: f64,
    }

    impl Default for MacosNativeViewManagerImpl {
        fn default() -> Self {
            Self {
                views: HashMap::new(),
                pending_events: Vec::new(),
                content_view: nil,
                dpi_factor: 1.0,
            }
        }
    }

    impl MacosNativeViewManagerImpl {
        pub fn new() -> Self {
            Self::default()
        }
    }
    
    // Implement the trait for the manager
    impl makepad_platform::os::apple::apple_native_view::AppleNativeViewImpl for MacosNativeViewManagerImpl {
        fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
            unsafe {
                let type_name = config.view_type.as_str();
                let properties = &config.properties;

                let view: ObjcId = match type_name {
                    "button" => Self::create_button(properties.get("label").map(|s| s.as_str()).unwrap_or("")),
                    "text_field" => Self::create_text_field(
                        properties.get("placeholder").map(|s| s.as_str()).unwrap_or(""),
                        properties.get("text").map(|s| s.as_str()).unwrap_or(""),
                    ),
                    "label" => Self::create_label(properties.get("text").map(|s| s.as_str()).unwrap_or("")),
                    "switch" => {
                        let on = properties.get("on").and_then(|v| v.parse::<bool>().ok()).unwrap_or(false);
                        Self::create_switch(on)
                    }
                    "slider" => {
                        let value = properties.get("value").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                        let min = properties.get("min").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                        let max = properties.get("max").and_then(|v| v.parse::<f64>().ok()).unwrap_or(1.0);
                        Self::create_slider(value, min, max)
                    }
                    "progress_indicator" => {
                        let progress = properties.get("progress").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                        Self::create_progress_indicator(progress)
                    }
                    _ => {
                        // Create a generic NSView for unknown custom types
                        let view: ObjcId = msg_send![class!(NSView), alloc];
                        let view: ObjcId = msg_send![view, init];
                        view
                    }
                };
                
                if view == nil {
                    return false;
                }
                
                // Set up off-screen rendering with CAMetalLayer
                let metal_layer = Self::setup_metal_layer(view, &config.frame, config.texture_scale);

                Self::apply_properties(type_name, view, properties);
                
                // Apply frame - note: we'll set the proper frame when added to content view
                let frame = config.frame;
                let ns_rect = NSRect {
                    origin: NSPoint { x: frame.pos.x, y: frame.pos.y },
                    size: NSSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![view, setFrame: ns_rect];
                
                // Ensure the view is clipped to its bounds and doesn't cover other content
                let () = msg_send![view, setClipsToBounds: YES];
                let () = msg_send![view, setWantsLayer: NO]; // Don't create a layer unless needed
                // Make the view non-opaque so it doesn't block Makepad rendering behind it
                // (though individual controls may still be opaque)
                let () = msg_send![view, setOpaque: NO];
                
                // Apply background color if specified  
                if let Some(_color) = config.background_color {
                    let () = msg_send![view, setWantsLayer: YES];
                    // Note: Full implementation needs CGColor creation helper
                    // For now, skip background color setup
                }
                
                // Calculate pixel dimensions
                let scale = (config.texture_scale * self.dpi_factor.max(1.0)).max(1.0);
                let width = (frame.size.x * scale) as usize;
                let height = (frame.size.y * scale) as usize;
                
                // Create container view for clipping
                let container_view: ObjcId = msg_send![class!(NSView), alloc];
                let container_view: ObjcId = msg_send![container_view, init];
                let () = msg_send![container_view, setWantsLayer: YES];
                let () = msg_send![container_view, setClipsToBounds: YES];
                let layer: ObjcId = msg_send![container_view, layer];
                let () = msg_send![layer, setMasksToBounds: YES];
                
                let clear_color: ObjcId = msg_send![class!(NSColor), clearColor];
                let cg_color: ObjcId = msg_send![clear_color, CGColor];
                let () = msg_send![layer, setBackgroundColor: cg_color];
                
                // Ensure the native view is layer-backed so it renders correctly inside the layer-backed container
                let () = msg_send![view, setWantsLayer: YES];

                // Add view to container
                let () = msg_send![container_view, addSubview: view];
                
                // Add to content view if available
                if self.content_view != nil {
                    // Get parent bounds to flip Y coordinate (macOS uses bottom-left origin)
                    let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                    let parent_height = parent_bounds.size.height;
                    
                    // Flip Y coordinate: macOS origin is bottom-left, but we work in top-left
                    let flipped_y = parent_height - frame.pos.y - frame.size.y;
                    let flipped_rect = NSRect {
                        origin: NSPoint { x: frame.pos.x, y: flipped_y },
                        size: NSSize { width: frame.size.x, height: frame.size.y },
                    };
                    let () = msg_send![container_view, setFrame: flipped_rect];
                    
                    // View frame inside container (initially full size)
                    let view_rect = NSRect {
                        origin: NSPoint { x: 0.0, y: 0.0 },
                        size: NSSize { width: frame.size.x, height: frame.size.y },
                    };
                    let () = msg_send![view, setFrame: view_rect];
                    
                    let () = msg_send![self.content_view, addSubview: container_view];
                }
                
                // Store state
                let mut props = properties.clone();
                Self::refresh_properties(type_name, view, &mut props);

                self.views.insert(id, MacosNativeViewState {
                    view,
                    container_view,
                    metal_layer,
                    frame,
                    clip: frame,
                    texture: None,
                    width,
                    height,
                    scale,
                    view_type: type_name.to_string(),
                    properties: props,
                });
                
                true
            }
        }
        
        fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
            let (view, view_type, clip) = if let Some(state) = self.views.get_mut(&id) {
                state.frame = config.frame;
                state.properties = config.properties.clone();
                (state.view, state.view_type.clone(), state.clip)
            } else {
                return false;
            };

            self.set_frame(id, config.frame, config.frame);
            Self::apply_properties(&view_type, view, &config.properties);

            if let Some(state) = self.views.get_mut(&id) {
                Self::refresh_properties(&view_type, state.view, &mut state.properties);
            }
            true
        }
        
        fn destroy_view(&mut self, id: NativeViewId) -> bool {
            if let Some(state) = self.views.remove(&id) {
                unsafe {
                    let () = msg_send![state.container_view, removeFromSuperview];
                    let () = msg_send![state.container_view, release];
                    let () = msg_send![state.view, release];
                }
                true
            } else {
                false
            }
        }
        
        fn set_frame(&mut self, id: NativeViewId, frame: Rect, clip: Rect) -> bool {
            if let Some(state) = self.views.get_mut(&id) {
                state.frame = frame;
                state.clip = clip;

                crate::log!("MacOS set_frame id: {:?} frame: {:?} clip: {:?}", id, frame, clip);

                unsafe {
                    if self.content_view != nil {
                        // Get parent bounds to flip Y coordinate
                        let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                        let parent_height = parent_bounds.size.height;
                        let scale = self.dpi_factor.max(1.0);
                        
                        // Container frame = clip rect (flipped Y)
                        let container_flipped_y = parent_height - clip.pos.y - clip.size.y;
                        let container_rect = NSRect {
                            origin: NSPoint { x: clip.pos.x, y: container_flipped_y },
                            size: NSSize { width: clip.size.x, height: clip.size.y },
                        };
                        let () = msg_send![state.container_view, setFrame: container_rect];

                        // View frame relative to container
                        let view_rel_x = frame.pos.x - clip.pos.x;
                        let view_rel_y = frame.pos.y - clip.pos.y;
                        let view_y_bl = clip.size.y - view_rel_y - frame.size.y;

                        let view_rect = NSRect {
                            origin: NSPoint { x: view_rel_x, y: view_y_bl.max(0.0) },
                            size: NSSize { width: frame.size.x, height: frame.size.y },
                        };
                        
                        crate::log!("  container_rect: {:?} view_rect: {:?}", container_rect, view_rect);

                        let () = msg_send![state.view, setFrame: view_rect];
                    } else {
                        // ...
                    }
                }
                
                true
            } else {
                false
            }
        }
        
        fn send_touch(&mut self, _event: NativeViewTouchEvent) -> bool {
            // Forward touch events to native views
            // This would need to convert Makepad touch events to NSEvent
            // For now, return false as this is complex
            false
        }
        
        fn get_texture(&self, _id: NativeViewId) -> Option<&NativeViewHandle> {
            // Get texture from metal layer
            // This would need to read from CAMetalLayer's drawable
            None
        }
        
        fn poll_events(&mut self) -> Vec<NativeViewEvent> {
            let mut events = std::mem::take(&mut self.pending_events);
            for (&id, state) in self.views.iter_mut() {
                events.extend(Self::collect_property_events(id, state));
            }
            events
        }
        
        fn set_content_view(&mut self, view: ObjcId, dpi_factor: f64) {
            self.content_view = view;
            self.dpi_factor = dpi_factor;
        }
    }
    
    // Helper methods for creating different view types (static methods)
    impl MacosNativeViewManagerImpl {
        fn apply_properties(view_type: &str, view: ObjcId, properties: &HashMap<String, String>) {
            unsafe {
                match view_type {
                    "button" => {
                        if let Some(label) = properties.get("label") {
                            let ns = str_to_nsstring(label);
                            let () = msg_send![view, setTitle: ns];
                        }
                    }
                    "text_field" | "label" => {
                        if let Some(text) = properties.get("text") {
                            let ns = str_to_nsstring(text);
                            let () = msg_send![view, setStringValue: ns];
                        }
                    }
                    "switch" => {
                        if let Some(on) = properties.get("on").and_then(|v| v.parse::<bool>().ok()) {
                            let state_value: i32 = if on { NS_CONTROL_STATE_VALUE_ON } else { NS_CONTROL_STATE_VALUE_OFF };
                            let () = msg_send![view, setState: state_value];
                        }
                    }
                    "slider" => {
                        if let Some(min) = properties.get("min").and_then(|v| v.parse::<f64>().ok()) {
                            let () = msg_send![view, setMinValue: min];
                        }
                        if let Some(max) = properties.get("max").and_then(|v| v.parse::<f64>().ok()) {
                            let () = msg_send![view, setMaxValue: max];
                        }
                        if let Some(value) = properties.get("value").and_then(|v| v.parse::<f64>().ok()) {
                            let () = msg_send![view, setDoubleValue: value];
                        }
                    }
                    "progress_indicator" => {
                        if let Some(progress) = properties.get("progress").and_then(|v| v.parse::<f64>().ok()) {
                            let () = msg_send![view, setDoubleValue: progress];
                        }
                    }
                    _ => {}
                }
            }
        }

        fn refresh_properties(view_type: &str, view: ObjcId, properties: &mut HashMap<String, String>) {
            unsafe {
                match view_type {
                    "button" => {
                        let highlighted: bool = msg_send![view, isHighlighted];
                        properties.insert("highlighted".into(), highlighted.to_string());
                    }
                    "text_field" | "label" => {
                        let value: ObjcId = msg_send![view, stringValue];
                        properties.insert("text".into(), nsstring_to_string(value));
                    }
                    "switch" => {
                        let state: i32 = msg_send![view, state];
                        properties.insert("on".into(), (state == NS_CONTROL_STATE_VALUE_ON).to_string());
                    }
                    "slider" => {
                        let value: f64 = msg_send![view, doubleValue];
                        properties.insert("value".into(), value.to_string());
                    }
                    "progress_indicator" => {
                        let value: f64 = msg_send![view, doubleValue];
                        properties.insert("progress".into(), value.to_string());
                    }
                    _ => {}
                }
            }
        }

        fn collect_property_events(id: NativeViewId, state: &mut MacosNativeViewState) -> Vec<NativeViewEvent> {
            let mut events = Vec::new();
            unsafe {
                match state.view_type.as_str() {
                    "button" => {
                        let highlighted: bool = msg_send![state.view, isHighlighted];
                        let highlighted_str = highlighted.to_string();
                        if state.properties.get("highlighted") != Some(&highlighted_str) {
                            // Treat the transition from pressed (true) back to false as a click.
                            let was_pressed = state.properties.get("highlighted").map(|v| v == "true").unwrap_or(false);
                            if was_pressed && !highlighted {
                                events.push(NativeViewEvent {
                                    id,
                                    kind: "button_tapped".into(),
                                    data: HashMap::new(),
                                });
                            }

                            state.properties.insert("highlighted".into(), highlighted_str);
                        }
                    }
                    "text_field" | "label" => {
                        let value: ObjcId = msg_send![state.view, stringValue];
                        let text = nsstring_to_string(value);
                        if state.properties.get("text") != Some(&text) {
                            state.properties.insert("text".into(), text.clone());
                            let mut data = HashMap::new();
                            data.insert("text".into(), text);
                            events.push(NativeViewEvent {
                                id,
                                kind: "text_changed".into(),
                                data,
                            });
                        }
                    }
                    "switch" => {
                        let state_value: i32 = msg_send![state.view, state];
                        let on = (state_value == NS_CONTROL_STATE_VALUE_ON).to_string();
                        if state.properties.get("on") != Some(&on) {
                            state.properties.insert("on".into(), on.clone());
                            let mut data = HashMap::new();
                            data.insert("on".into(), on);
                            events.push(NativeViewEvent {
                                id,
                                kind: "switch_changed".into(),
                                data,
                            });
                        }
                    }
                    "slider" => {
                        let value: f64 = msg_send![state.view, doubleValue];
                        let value_str = value.to_string();
                        if state.properties.get("value") != Some(&value_str) {
                            state.properties.insert("value".into(), value_str.clone());
                            let mut data = HashMap::new();
                            data.insert("value".into(), value_str);
                            events.push(NativeViewEvent {
                                id,
                                kind: "slider_changed".into(),
                                data,
                            });
                        }
                    }
                    "progress_indicator" => {
                        let value: f64 = msg_send![state.view, doubleValue];
                        let value_str = value.to_string();
                        if state.properties.get("progress") != Some(&value_str) {
                            state.properties.insert("progress".into(), value_str.clone());
                            let mut data = HashMap::new();
                            data.insert("progress".into(), value_str);
                            events.push(NativeViewEvent {
                                id,
                                kind: "progress_changed".into(),
                                data,
                            });
                        }
                    }
                    _ => {}
                }
            }
            events
        }

        fn create_button(label: &str) -> ObjcId {
            unsafe {
                let button: ObjcId = msg_send![class!(NSButton), alloc];
                let button: ObjcId = msg_send![button, init];
                let ns_string = str_to_nsstring(label);
                let () = msg_send![button, setTitle: ns_string];
                // Default momentary push button keeps native look/behavior
                button
            }
        }
        
        fn create_text_field(placeholder: &str, text: &str) -> ObjcId {
            unsafe {
                let field: ObjcId = msg_send![class!(NSTextField), alloc];
                let field: ObjcId = msg_send![field, init];
                let placeholder_str = str_to_nsstring(placeholder);
                let text_str = str_to_nsstring(text);
                let () = msg_send![field, setPlaceholderString: placeholder_str];
                let () = msg_send![field, setStringValue: text_str];
                field
            }
        }
        
        fn create_label(text: &str) -> ObjcId {
            unsafe {
                let label: ObjcId = msg_send![class!(NSTextField), alloc];
                let label: ObjcId = msg_send![label, init];
                let text_str = str_to_nsstring(text);
                let () = msg_send![label, setStringValue: text_str];
                let () = msg_send![label, setEditable: NO];
                let () = msg_send![label, setBezeled: NO];
                let () = msg_send![label, setDrawsBackground: NO];
                label
            }
        }
        
        fn create_switch(_on: bool) -> ObjcId {
            unsafe {
                let switch_view: ObjcId = msg_send![class!(NSSwitch), alloc];
                let switch_view: ObjcId = msg_send![switch_view, init];
                // Note: Full implementation needs NSControlStateValue constants
                // For now, just create the switch
                switch_view
            }
        }
        
        fn create_slider(value: f64, min: f64, max: f64) -> ObjcId {
            unsafe {
                let slider: ObjcId = msg_send![class!(NSSlider), alloc];
                let slider: ObjcId = msg_send![slider, init];
                let () = msg_send![slider, setMinValue: min];
                let () = msg_send![slider, setMaxValue: max];
                let () = msg_send![slider, setDoubleValue: value];
                slider
            }
        }
        
        fn create_progress_indicator(progress: f64) -> ObjcId {
            unsafe {
                let indicator: ObjcId = msg_send![class!(NSProgressIndicator), alloc];
                let indicator: ObjcId = msg_send![indicator, init];
                // Note: Full implementation needs NSProgressIndicatorStyleBar constant
                let () = msg_send![indicator, setIndeterminate: NO];
                let () = msg_send![indicator, setDoubleValue: progress];
                indicator
            }
        }
        
        fn setup_metal_layer(_view: ObjcId, _frame: &Rect, _scale: f64) -> ObjcId {
            // For now, return nil as texture sharing is complex
            // Full implementation would create CAMetalLayer and set it up
            nil
        }
    }
}

// Make it Send + Sync (ObjcId is just a pointer, safe to send)
unsafe impl Send for macos_impl::MacosNativeViewManagerImpl {}
unsafe impl Sync for macos_impl::MacosNativeViewManagerImpl {}

#[cfg(target_os = "macos")]
pub use macos_impl::*;

#[cfg(target_os = "macos")]
/// Initialize macOS native view support
/// This registers the full implementation with the platform
pub fn init(cx: &mut makepad_draw::Cx) {
    let impl_ = Box::new(MacosNativeViewManagerImpl::new());
    cx.set_macos_native_view_impl(impl_);
}
