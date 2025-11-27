// macOS native view implementation
// This creates and manages native NSViews for embedding in Makepad

use {
    std::collections::HashMap,
    crate::{
        makepad_math::*,
        makepad_objc_sys::runtime::{nil, ObjcId},
        native_view::{
            NativeViewId,
            NativeViewConfig,
            NativeViewType,
            NativeViewEvent,
            NativeViewTouchEvent,
            NativeViewHandle,
        },
        os::apple::apple_sys::*,
        os::apple::apple_util::str_to_nsstring,
    },
};

/// macOS-specific native view state
struct MacosNativeViewState {
    /// The native NSView
    view: ObjcId,
    /// Current frame
    frame: Rect,
    /// Width in pixels
    width: usize,
    /// Height in pixels
    height: usize,
    /// Scale factor
    scale: f64,
}

/// Manager for macOS native views
pub struct MacosNativeViewManager {
    views: HashMap<NativeViewId, MacosNativeViewState>,
    pending_events: Vec<NativeViewEvent>,
    pub content_view: ObjcId,
    pub dpi_factor: f64,
}

impl Default for MacosNativeViewManager {
    fn default() -> Self {
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
            content_view: nil,
            dpi_factor: 1.0,
        }
    }
}

impl MacosNativeViewManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_content_view(&mut self, view: ObjcId, dpi: f64) {
        self.content_view = view;
        self.dpi_factor = dpi;
    }
    
    pub fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        unsafe {
            let view: ObjcId = match &config.view_type {
                NativeViewType::Button { label } => {
                    self.create_button(label)
                }
                NativeViewType::TextField { placeholder, text } => {
                    self.create_text_field(placeholder, text)
                }
                NativeViewType::Label { text } => {
                    self.create_label(text)
                }
                NativeViewType::Switch { on } => {
                    self.create_switch(*on)
                }
                NativeViewType::Slider { value, min, max } => {
                    self.create_slider(*value, *min, *max)
                }
                NativeViewType::ProgressIndicator { progress } => {
                    self.create_progress_indicator(*progress)
                }
                NativeViewType::Custom { .. } => {
                    let view: ObjcId = msg_send![class!(NSView), alloc];
                    let view: ObjcId = msg_send![view, init];
                    view
                }
            };
            
            if view == nil {
                return false;
            }
            
            let frame = config.frame;
            let ns_rect = NSRect {
                origin: NSPoint { x: frame.pos.x, y: frame.pos.y },
                size: NSSize { width: frame.size.x, height: frame.size.y },
            };
            let () = msg_send![view, setFrame: ns_rect];
            
            let scale = config.texture_scale.max(1.0);
            let width = (frame.size.x * scale) as usize;
            let height = (frame.size.y * scale) as usize;
            
            // Add to content view if available
            if self.content_view != nil {
                let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                let parent_height = parent_bounds.size.height;
                
                // Flip Y coordinate: macOS origin is bottom-left
                let flipped_y = parent_height - frame.pos.y - frame.size.y;
                let flipped_rect = NSRect {
                    origin: NSPoint { x: frame.pos.x, y: flipped_y },
                    size: NSSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![view, setFrame: flipped_rect];
                let () = msg_send![self.content_view, addSubview: view];
            }
            
            self.views.insert(id, MacosNativeViewState {
                view,
                frame,
                width,
                height,
                scale,
            });
            
            true
        }
    }
    
    pub fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            state.frame = config.frame;
            self.set_frame(id, config.frame)
        } else {
            false
        }
    }
    
    pub fn destroy_view(&mut self, id: NativeViewId) -> bool {
        if let Some(state) = self.views.remove(&id) {
            unsafe {
                let () = msg_send![state.view, removeFromSuperview];
                let () = msg_send![state.view, release];
            }
            true
        } else {
            false
        }
    }
    
    pub fn set_frame(&mut self, id: NativeViewId, frame: Rect) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            state.frame = frame;
            unsafe {
                if self.content_view != nil {
                    let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                    let parent_height = parent_bounds.size.height;
                    let flipped_y = parent_height - frame.pos.y - frame.size.y;
                    let flipped_rect = NSRect {
                        origin: NSPoint { x: frame.pos.x, y: flipped_y },
                        size: NSSize { width: frame.size.x, height: frame.size.y },
                    };
                    let () = msg_send![state.view, setFrame: flipped_rect];
                } else {
                    let ns_rect = NSRect {
                        origin: NSPoint { x: frame.pos.x, y: frame.pos.y },
                        size: NSSize { width: frame.size.x, height: frame.size.y },
                    };
                    let () = msg_send![state.view, setFrame: ns_rect];
                }
            }
            true
        } else {
            false
        }
    }
    
    pub fn send_touch(&mut self, _event: NativeViewTouchEvent) -> bool {
        false
    }
    
    pub fn get_texture(&self, _id: NativeViewId) -> Option<&NativeViewHandle> {
        None
    }
    
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        std::mem::take(&mut self.pending_events)
    }
    
    fn create_button(&self, label: &str) -> ObjcId {
        unsafe {
            let button: ObjcId = msg_send![class!(NSButton), alloc];
            let button: ObjcId = msg_send![button, init];
            let ns_string = str_to_nsstring(label);
            let () = msg_send![button, setTitle: ns_string];
            button
        }
    }
    
    fn create_text_field(&self, placeholder: &str, text: &str) -> ObjcId {
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
    
    fn create_label(&self, text: &str) -> ObjcId {
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
    
    fn create_switch(&self, _on: bool) -> ObjcId {
        unsafe {
            let switch_view: ObjcId = msg_send![class!(NSSwitch), alloc];
            let switch_view: ObjcId = msg_send![switch_view, init];
            switch_view
        }
    }
    
    fn create_slider(&self, value: f64, min: f64, max: f64) -> ObjcId {
        unsafe {
            let slider: ObjcId = msg_send![class!(NSSlider), alloc];
            let slider: ObjcId = msg_send![slider, init];
            let () = msg_send![slider, setMinValue: min];
            let () = msg_send![slider, setMaxValue: max];
            let () = msg_send![slider, setDoubleValue: value];
            slider
        }
    }
    
    fn create_progress_indicator(&self, progress: f64) -> ObjcId {
        unsafe {
            let indicator: ObjcId = msg_send![class!(NSProgressIndicator), alloc];
            let indicator: ObjcId = msg_send![indicator, init];
            let () = msg_send![indicator, setIndeterminate: NO];
            let () = msg_send![indicator, setDoubleValue: progress];
            indicator
        }
    }
}
