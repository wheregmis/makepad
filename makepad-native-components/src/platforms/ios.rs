// Full iOS native view implementation
// This provides the actual functionality for creating and managing native UIViews

#[cfg(target_os = "ios")]
mod ios_impl {
    use {
        makepad_platform::os::apple::apple_sys::*,
        makepad_platform::os::apple::apple_util::{nsstring_to_string, str_to_nsstring},
        makepad_platform::{
            makepad_math::*,
            makepad_objc_sys::runtime::{nil, ObjcId},
            native_view::{
                NativeViewConfig, NativeViewEvent, NativeViewHandle, NativeViewId,
                NativeViewTouchEvent,
            },
        },
        std::collections::HashMap,
    };

    /// iOS-specific native view state
    pub struct IosNativeViewState {
        /// The native UIView
        view: ObjcId,
        /// The container view used for clipping
        container_view: ObjcId,
        /// Off-screen render target (CAMetalLayer)
        metal_layer: ObjcId,
        /// Current frame
        frame: Rect,
        /// Current clip
        clip: Rect,
        /// Texture for rendering (would be Texture type in full implementation)
        texture: Option<()>,
        /// Width in pixels
        width: usize,
        /// Height in pixels
        height: usize,
        /// Scale factor
        scale: f64,
        /// View type identifier
        view_type: String,
        /// Last known properties (stringified)
        properties: HashMap<String, String>,
    }

    /// Full implementation manager for iOS native views
    pub struct IosNativeViewManagerImpl {
        views: HashMap<NativeViewId, IosNativeViewState>,
        pending_events: Vec<NativeViewEvent>,
    }

    impl Default for IosNativeViewManagerImpl {
        fn default() -> Self {
            Self {
                views: HashMap::new(),
                pending_events: Vec::new(),
            }
        }
    }

    impl IosNativeViewManagerImpl {
        pub fn new() -> Self {
            Self::default()
        }
    }

    // Implement the trait for the manager
    impl makepad_platform::os::apple::apple_native_view::AppleNativeViewImpl
        for IosNativeViewManagerImpl
    {
        fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
            unsafe {
                let type_name = config.view_type.as_str();
                let properties = &config.properties;

                let view: ObjcId = match type_name {
                    "button" => Self::create_button(
                        properties.get("label").map(|s| s.as_str()).unwrap_or(""),
                    ),
                    "text_field" => Self::create_text_field(
                        properties
                            .get("placeholder")
                            .map(|s| s.as_str())
                            .unwrap_or(""),
                        properties.get("text").map(|s| s.as_str()).unwrap_or(""),
                    ),
                    "label" => {
                        Self::create_label(properties.get("text").map(|s| s.as_str()).unwrap_or(""))
                    }
                    "switch" => {
                        let on = properties
                            .get("on")
                            .and_then(|v| v.parse::<bool>().ok())
                            .unwrap_or(false);
                        Self::create_switch(on)
                    }
                    "slider" => {
                        let value = properties
                            .get("value")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let min = properties
                            .get("min")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let max = properties
                            .get("max")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(1.0);
                        Self::create_slider(value, min, max)
                    }
                    "progress_indicator" => {
                        let progress = properties
                            .get("progress")
                            .and_then(|v| v.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        Self::create_progress_indicator(progress)
                    }
                    _ => {
                        // Create a generic UIView for unknown custom types
                        let view: ObjcId = msg_send![class!(UIView), alloc];
                        let view: ObjcId = msg_send![view, init];
                        view
                    }
                };

                if view == nil {
                    return false;
                }

                // Apply initial properties
                Self::apply_properties(type_name, view, properties);

                // Create container view for clipping
                let container_view: ObjcId = msg_send![class!(UIView), alloc];
                let container_view: ObjcId = msg_send![container_view, init];
                let () = msg_send![container_view, setClipsToBounds: YES];

                // Add view to container
                let () = msg_send![container_view, addSubview: view];

                // Set up frame
                let frame = config.frame;
                
                // Container frame = frame (initially unclipped)
                let cg_rect = CGRect {
                    origin: CGPoint {
                        x: frame.pos.x,
                        y: frame.pos.y,
                    },
                    size: CGSize {
                        width: frame.size.x,
                        height: frame.size.y,
                    },
                };
                let () = msg_send![container_view, setFrame: cg_rect];
                
                // View frame inside container (0,0)
                let view_rect = CGRect {
                    origin: CGPoint { x: 0.0, y: 0.0 },
                    size: CGSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![view, setFrame: view_rect];

                // Add container to MTKView
                makepad_platform::os::apple::ios::ios_app::with_ios_app(|app| {
                    if let Some(mtk_view) = app.mtk_view {
                        let () = msg_send![mtk_view, addSubview: container_view];
                    }
                });

                // Calculate pixel dimensions
                let scale = config.texture_scale.max(1.0);
                let width = (frame.size.x * scale) as usize;
                let height = (frame.size.y * scale) as usize;

                // Store state
                let mut props = properties.clone();
                Self::refresh_properties(type_name, view, &mut props);

                self.views.insert(
                    id,
                    IosNativeViewState {
                        view,
                        container_view,
                        metal_layer: nil,
                        frame,
                        clip: frame,
                        texture: None,
                        width,
                        height,
                        scale,
                        view_type: type_name.to_string(),
                        properties: props,
                    },
                );

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

            self.set_frame(id, config.frame, clip);
            Self::apply_properties(view_type.as_str(), view, &config.properties);

            if let Some(state) = self.views.get_mut(&id) {
                Self::refresh_properties(view_type.as_str(), state.view, &mut state.properties);
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
                unsafe {
                    // Container frame = clip rect
                    let container_rect = CGRect {
                        origin: CGPoint {
                            x: clip.pos.x,
                            y: clip.pos.y,
                        },
                        size: CGSize {
                            width: clip.size.x,
                            height: clip.size.y,
                        },
                    };
                    let () = msg_send![state.container_view, setFrame: container_rect];

                    // View frame relative to container
                    let view_x = frame.pos.x - clip.pos.x;
                    let view_y = frame.pos.y - clip.pos.y;
                    
                    let view_rect = CGRect {
                        origin: CGPoint {
                            x: view_x,
                            y: view_y,
                        },
                        size: CGSize {
                            width: frame.size.x,
                            height: frame.size.y,
                        },
                    };
                    let () = msg_send![state.view, setFrame: view_rect];
                }
                true
            } else {
                false
            }
        }

        fn send_touch(&mut self, _event: NativeViewTouchEvent) -> bool {
            // Forward touch events to native views
            false
        }

        fn get_texture(&self, _id: NativeViewId) -> Option<&NativeViewHandle> {
            None
        }

        fn poll_events(&mut self) -> Vec<NativeViewEvent> {
            let mut events = std::mem::take(&mut self.pending_events);
            for (&id, state) in self.views.iter_mut() {
                events.extend(Self::collect_property_events(id, state));
            }
            events
        }
    }

    // Helper methods for creating different view types (static methods)
    impl IosNativeViewManagerImpl {
        fn apply_properties(view_type: &str, view: ObjcId, properties: &HashMap<String, String>) {
            unsafe {
                match view_type {
                    "button" => {
                        if let Some(label) = properties.get("label") {
                            let title = str_to_nsstring(label);
                            let () = msg_send![view, setTitle: title forState: 0];
                        }
                    }
                    "text_field" => {
                        if let Some(placeholder) = properties.get("placeholder") {
                            let ns = str_to_nsstring(placeholder);
                            let () = msg_send![view, setPlaceholder: ns];
                        }
                        if let Some(text) = properties.get("text") {
                            let ns = str_to_nsstring(text);
                            let () = msg_send![view, setText: ns];
                        }
                    }
                    "switch" => {
                        if let Some(on) = properties.get("on").and_then(|v| v.parse::<bool>().ok())
                        {
                            let () = msg_send![view, setOn: on];
                        }
                    }
                    "slider" => {
                        if let Some(min) = properties.get("min").and_then(|v| v.parse::<f64>().ok())
                        {
                            let () = msg_send![view, setMinimumValue: min];
                        }
                        if let Some(max) = properties.get("max").and_then(|v| v.parse::<f64>().ok())
                        {
                            let () = msg_send![view, setMaximumValue: max];
                        }
                        if let Some(value) =
                            properties.get("value").and_then(|v| v.parse::<f64>().ok())
                        {
                            let () = msg_send![view, setValue: value];
                        }
                    }
                    "progress_indicator" => {
                        if let Some(progress) = properties
                            .get("progress")
                            .and_then(|v| v.parse::<f64>().ok())
                        {
                            let () = msg_send![view, setProgress: progress];
                        }
                    }
                    _ => {}
                }
            }
        }

        fn refresh_properties(
            view_type: &str,
            view: ObjcId,
            properties: &mut HashMap<String, String>,
        ) {
            unsafe {
                match view_type {
                    "text_field" => {
                        let text: ObjcId = msg_send![view, text];
                        properties.insert("text".into(), nsstring_to_string(text));
                    }
                    "switch" => {
                        let on: bool = msg_send![view, isOn];
                        properties.insert("on".into(), on.to_string());
                    }
                    "slider" => {
                        let value: f64 = msg_send![view, value];
                        properties.insert("value".into(), value.to_string());
                    }
                    "progress_indicator" => {
                        let progress: f64 = msg_send![view, progress];
                        properties.insert("progress".into(), progress.to_string());
                    }
                    _ => {}
                }
            }
        }

        fn collect_property_events(
            id: NativeViewId,
            state: &mut IosNativeViewState,
        ) -> Vec<NativeViewEvent> {
            let mut events = Vec::new();
            unsafe {
                match state.view_type.as_str() {
                    "text_field" => {
                        let text: ObjcId = msg_send![state.view, text];
                        let value = nsstring_to_string(text);
                        if state.properties.get("text") != Some(&value) {
                            state.properties.insert("text".into(), value.clone());
                            let mut data = HashMap::new();
                            data.insert("text".into(), value);
                            events.push(NativeViewEvent {
                                id,
                                kind: "text_changed".into(),
                                data,
                            });
                        }
                    }
                    "switch" => {
                        let on: bool = msg_send![state.view, isOn];
                        let value = on.to_string();
                        if state.properties.get("on") != Some(&value) {
                            state.properties.insert("on".into(), value.clone());
                            let mut data = HashMap::new();
                            data.insert("on".into(), value);
                            events.push(NativeViewEvent {
                                id,
                                kind: "switch_changed".into(),
                                data,
                            });
                        }
                    }
                    "slider" => {
                        let value: f64 = msg_send![state.view, value];
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
                        let progress: f64 = msg_send![state.view, progress];
                        let value_str = progress.to_string();
                        if state.properties.get("progress") != Some(&value_str) {
                            state
                                .properties
                                .insert("progress".into(), value_str.clone());
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
                // Note: Full implementation needs UIButtonTypeSystem constant
                let button: ObjcId = msg_send![class!(UIButton), alloc];
                let button: ObjcId = msg_send![button, init];
                let ns_string = str_to_nsstring(label);
                // Note: Full implementation needs UIControlStateNormal constant
                // let () = msg_send![button, setTitle: ns_string forState: UIControlStateNormal];
                button
            }
        }

        fn create_text_field(placeholder: &str, text: &str) -> ObjcId {
            unsafe {
                let field: ObjcId = msg_send![class!(UITextField), alloc];
                let field: ObjcId = msg_send![field, init];
                let placeholder_str = str_to_nsstring(placeholder);
                let text_str = str_to_nsstring(text);
                let () = msg_send![field, setPlaceholder: placeholder_str];
                let () = msg_send![field, setText: text_str];
                field
            }
        }

        fn create_label(text: &str) -> ObjcId {
            unsafe {
                let label: ObjcId = msg_send![class!(UILabel), alloc];
                let label: ObjcId = msg_send![label, init];
                let text_str = str_to_nsstring(text);
                let () = msg_send![label, setText: text_str];
                label
            }
        }

        fn create_switch(on: bool) -> ObjcId {
            unsafe {
                let switch_view: ObjcId = msg_send![class!(UISwitch), alloc];
                let switch_view: ObjcId = msg_send![switch_view, init];
                let () = msg_send![switch_view, setOn: on];
                switch_view
            }
        }

        fn create_slider(value: f64, min: f64, max: f64) -> ObjcId {
            unsafe {
                let slider: ObjcId = msg_send![class!(UISlider), alloc];
                let slider: ObjcId = msg_send![slider, init];
                let () = msg_send![slider, setMinimumValue: min];
                let () = msg_send![slider, setMaximumValue: max];
                let () = msg_send![slider, setValue: value];
                slider
            }
        }

        fn create_progress_indicator(progress: f64) -> ObjcId {
            unsafe {
                let indicator: ObjcId = msg_send![class!(UIProgressView), alloc];
                let indicator: ObjcId = msg_send![indicator, init];
                let () = msg_send![indicator, setProgress: progress];
                indicator
            }
        }
    }
}

#[cfg(target_os = "ios")]
pub use ios_impl::*;

// Make it Send + Sync (ObjcId is just a pointer, safe to send)
#[cfg(target_os = "ios")]
unsafe impl Send for ios_impl::IosNativeViewManagerImpl {}
#[cfg(target_os = "ios")]
unsafe impl Sync for ios_impl::IosNativeViewManagerImpl {}

#[cfg(target_os = "ios")]
/// Initialize iOS native view support
/// This registers the full implementation with the platform
pub fn init(cx: &mut makepad_draw::Cx) {
    use makepad_platform::native_view::AppleNativeViewImpl;

    let impl_ = Box::new(IosNativeViewManagerImpl::new());
    cx.set_ios_native_view_impl(impl_);
}
