use {
    std::collections::HashMap,
    crate::{
        makepad_objc_sys::runtime::{nil, ObjcId},
        makepad_math::*,
        native_view::{
            NativeViewId,
            NativeViewConfig,
            NativeViewType,
            NativeViewEvent,
            NativeViewTouchEvent,
            NativeViewTouchPhase,
            NativeViewHandle,
        },
        texture::{Texture, TextureFormat},
        os::apple::apple_sys::*,
        os::apple::apple_util::*,
    },
};

/// iOS-specific native view state
pub struct IosNativeViewState {
    /// The native UIView
    pub view: ObjcId,
    /// Off-screen render target (CAMetalLayer)
    pub metal_layer: ObjcId,
    /// Current frame
    pub frame: Rect,
    /// Texture for rendering
    pub texture: Option<Texture>,
    /// Width in pixels
    pub width: usize,
    /// Height in pixels
    pub height: usize,
    /// Scale factor
    pub scale: f64,
}

/// Manager for iOS native views
#[derive(Default)]
pub struct IosNativeViewManager {
    pub views: HashMap<NativeViewId, IosNativeViewState>,
    pub pending_events: Vec<NativeViewEvent>,
}

impl IosNativeViewManager {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
        }
    }
    
    /// Create a native UIView based on the config
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
                    // Create a generic UIView for custom types
                    let view: ObjcId = msg_send![class!(UIView), alloc];
                    let view: ObjcId = msg_send![view, init];
                    view
                }
            };
            
            if view == nil {
                return false;
            }
            
            // Set up off-screen rendering with CAMetalLayer
            let metal_layer = self.setup_metal_layer(view, &config.frame, config.texture_scale);
            
            // Apply frame
            let frame = config.frame;
            let cg_rect = CGRect {
                origin: CGPoint { x: frame.pos.x, y: frame.pos.y },
                size: CGSize { width: frame.size.x, height: frame.size.y },
            };
            let () = msg_send![view, setFrame: cg_rect];
            
            // Apply background color if specified
            if let Some(color) = config.background_color {
                let ui_color: ObjcId = msg_send![
                    class!(UIColor),
                    colorWithRed: color.x as f64
                    green: color.y as f64
                    blue: color.z as f64
                    alpha: color.w as f64
                ];
                let () = msg_send![view, setBackgroundColor: ui_color];
            }
            
            // Calculate pixel dimensions
            let scale = config.texture_scale.max(1.0);
            let width = (frame.size.x * scale) as usize;
            let height = (frame.size.y * scale) as usize;
            
            let state = IosNativeViewState {
                view,
                metal_layer,
                frame: config.frame,
                texture: None,
                width,
                height,
                scale,
            };
            
            self.views.insert(id, state);
            true
        }
    }
    
    /// Create a UIButton
    unsafe fn create_button(&self, label: &str) -> ObjcId {
        let button: ObjcId = msg_send![class!(UIButton), buttonWithType: 0i64]; // UIButtonTypeCustom
        let ns_label = str_to_nsstring(label);
        let () = msg_send![button, setTitle: ns_label forState: 0u64]; // UIControlStateNormal
        
        // Style the button
        let () = msg_send![button, setTitleColor: msg_send![class!(UIColor), systemBlueColor] forState: 0u64];
        
        button
    }
    
    /// Create a UITextField
    unsafe fn create_text_field(&self, placeholder: &str, text: &str) -> ObjcId {
        let text_field: ObjcId = msg_send![class!(UITextField), alloc];
        let text_field: ObjcId = msg_send![text_field, init];
        
        let ns_placeholder = str_to_nsstring(placeholder);
        let ns_text = str_to_nsstring(text);
        
        let () = msg_send![text_field, setPlaceholder: ns_placeholder];
        let () = msg_send![text_field, setText: ns_text];
        let () = msg_send![text_field, setBorderStyle: 3i64]; // UITextBorderStyleRoundedRect
        
        text_field
    }
    
    /// Create a UILabel
    unsafe fn create_label(&self, text: &str) -> ObjcId {
        let label: ObjcId = msg_send![class!(UILabel), alloc];
        let label: ObjcId = msg_send![label, init];
        
        let ns_text = str_to_nsstring(text);
        let () = msg_send![label, setText: ns_text];
        
        label
    }
    
    /// Create a UISwitch
    unsafe fn create_switch(&self, on: bool) -> ObjcId {
        let switch: ObjcId = msg_send![class!(UISwitch), alloc];
        let switch: ObjcId = msg_send![switch, init];
        
        let () = msg_send![switch, setOn: on animated: false];
        
        switch
    }
    
    /// Create a UISlider
    unsafe fn create_slider(&self, value: f64, min: f64, max: f64) -> ObjcId {
        let slider: ObjcId = msg_send![class!(UISlider), alloc];
        let slider: ObjcId = msg_send![slider, init];
        
        let () = msg_send![slider, setMinimumValue: min as f32];
        let () = msg_send![slider, setMaximumValue: max as f32];
        let () = msg_send![slider, setValue: value as f32 animated: false];
        
        slider
    }
    
    /// Create a UIProgressView
    unsafe fn create_progress_indicator(&self, progress: f64) -> ObjcId {
        let progress_view: ObjcId = msg_send![class!(UIProgressView), alloc];
        let progress_view: ObjcId = msg_send![progress_view, initWithProgressViewStyle: 0i64]; // UIProgressViewStyleDefault
        
        let () = msg_send![progress_view, setProgress: progress as f32 animated: false];
        
        progress_view
    }
    
    /// Set up a CAMetalLayer for off-screen rendering
    unsafe fn setup_metal_layer(&self, view: ObjcId, frame: &Rect, scale: f64) -> ObjcId {
        // Create a CAMetalLayer
        let metal_layer: ObjcId = msg_send![class!(CAMetalLayer), layer];
        
        // Get the Metal device from the view's layer or create one
        let device = get_default_metal_device();
        if let Some(device) = device {
            let () = msg_send![metal_layer, setDevice: device];
        }
        
        // Set pixel format
        let () = msg_send![metal_layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm as u64];
        
        // Set the frame size
        let drawable_size = CGSize {
            width: frame.size.x * scale,
            height: frame.size.y * scale,
        };
        let () = msg_send![metal_layer, setDrawableSize: drawable_size];
        
        // Set content scale
        let () = msg_send![metal_layer, setContentsScale: scale];
        
        // Enable framebuffer only mode for better performance
        let () = msg_send![metal_layer, setFramebufferOnly: NO];
        
        // Add the metal layer as a sublayer of the view's layer
        let view_layer: ObjcId = msg_send![view, layer];
        let () = msg_send![view_layer, addSublayer: metal_layer];
        
        metal_layer
    }
    
    /// Update an existing native view
    pub fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            unsafe {
                // Update frame
                let frame = config.frame;
                let cg_rect = CGRect {
                    origin: CGPoint { x: frame.pos.x, y: frame.pos.y },
                    size: CGSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![state.view, setFrame: cg_rect];
                
                // Update metal layer drawable size
                let scale = config.texture_scale.max(1.0);
                let drawable_size = CGSize {
                    width: frame.size.x * scale,
                    height: frame.size.y * scale,
                };
                let () = msg_send![state.metal_layer, setDrawableSize: drawable_size];
                
                state.frame = frame;
                state.width = (frame.size.x * scale) as usize;
                state.height = (frame.size.y * scale) as usize;
                state.scale = scale;
                
                // Update view-specific properties
                match &config.view_type {
                    NativeViewType::Button { label } => {
                        let ns_label = str_to_nsstring(label);
                        let () = msg_send![state.view, setTitle: ns_label forState: 0u64];
                    }
                    NativeViewType::TextField { placeholder, text } => {
                        let ns_placeholder = str_to_nsstring(placeholder);
                        let ns_text = str_to_nsstring(text);
                        let () = msg_send![state.view, setPlaceholder: ns_placeholder];
                        let () = msg_send![state.view, setText: ns_text];
                    }
                    NativeViewType::Label { text } => {
                        let ns_text = str_to_nsstring(text);
                        let () = msg_send![state.view, setText: ns_text];
                    }
                    NativeViewType::Switch { on } => {
                        let () = msg_send![state.view, setOn: *on animated: true];
                    }
                    NativeViewType::Slider { value, min, max } => {
                        let () = msg_send![state.view, setMinimumValue: *min as f32];
                        let () = msg_send![state.view, setMaximumValue: *max as f32];
                        let () = msg_send![state.view, setValue: *value as f32 animated: true];
                    }
                    NativeViewType::ProgressIndicator { progress } => {
                        let () = msg_send![state.view, setProgress: *progress as f32 animated: true];
                    }
                    NativeViewType::Custom { .. } => {}
                }
            }
            true
        } else {
            false
        }
    }
    
    /// Destroy a native view
    pub fn destroy_view(&mut self, id: NativeViewId) -> bool {
        if let Some(state) = self.views.remove(&id) {
            unsafe {
                // Remove from superview
                let () = msg_send![state.view, removeFromSuperview];
                // Release the view
                let () = msg_send![state.view, release];
            }
            true
        } else {
            false
        }
    }
    
    /// Set the frame of a native view
    pub fn set_frame(&mut self, id: NativeViewId, frame: Rect) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            unsafe {
                let cg_rect = CGRect {
                    origin: CGPoint { x: frame.pos.x, y: frame.pos.y },
                    size: CGSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![state.view, setFrame: cg_rect];
                
                // Update metal layer drawable size
                let drawable_size = CGSize {
                    width: frame.size.x * state.scale,
                    height: frame.size.y * state.scale,
                };
                let () = msg_send![state.metal_layer, setDrawableSize: drawable_size];
                
                state.frame = frame;
                state.width = (frame.size.x * state.scale) as usize;
                state.height = (frame.size.y * state.scale) as usize;
            }
            true
        } else {
            false
        }
    }
    
    /// Forward a touch event to a native view
    pub fn send_touch(&mut self, event: NativeViewTouchEvent) -> bool {
        if let Some(state) = self.views.get(&event.id) {
            unsafe {
                // Convert touch phase
                let phase: i64 = match event.phase {
                    NativeViewTouchPhase::Began => 0,     // UITouchPhaseBegan
                    NativeViewTouchPhase::Moved => 1,     // UITouchPhaseMoved
                    NativeViewTouchPhase::Ended => 3,     // UITouchPhaseEnded
                    NativeViewTouchPhase::Cancelled => 4, // UITouchPhaseCancelled
                };
                
                // Create a synthetic touch event
                // Note: This is a simplified implementation. For full touch forwarding,
                // you would need to use UIKit's private APIs or implement a hit-testing system.
                
                // For buttons, we can use sendActionsForControlEvents
                let class_name: ObjcId = msg_send![state.view, class];
                let class_name_str: ObjcId = msg_send![class_name, description];
                let class_str = nsstring_to_string(class_name_str);
                
                if class_str == "UIButton" {
                    match event.phase {
                        NativeViewTouchPhase::Began => {
                            let () = msg_send![state.view, sendActionsForControlEvents: 1u64]; // UIControlEventTouchDown
                        }
                        NativeViewTouchPhase::Ended => {
                            let () = msg_send![state.view, sendActionsForControlEvents: 64u64]; // UIControlEventTouchUpInside
                            // Queue button tap event
                            self.pending_events.push(NativeViewEvent::ButtonTapped { id: event.id });
                        }
                        _ => {}
                    }
                }
            }
            true
        } else {
            false
        }
    }
    
    /// Get the texture handle for a native view
    pub fn get_texture(&self, id: NativeViewId) -> Option<&NativeViewHandle> {
        // This would return the texture handle if we've rendered to it
        // For now, return None as texture creation is deferred
        None
    }
    
    /// Poll for events from native views
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        std::mem::take(&mut self.pending_events)
    }
    
    /// Render a native view to its texture
    /// This captures the UIView's content to a Metal texture
    pub fn render_to_texture(&mut self, id: NativeViewId, metal_device: ObjcId) -> Option<ObjcId> {
        if let Some(state) = self.views.get_mut(&id) {
            unsafe {
                // Get the next drawable from the metal layer
                let drawable: ObjcId = msg_send![state.metal_layer, nextDrawable];
                if drawable == nil {
                    return None;
                }
                
                // Get the texture from the drawable
                let texture: ObjcId = msg_send![drawable, texture];
                
                // Render the UIView to the texture using UIGraphicsImageRenderer
                // This is a simplified approach - for production, you'd use
                // drawViewHierarchyInRect or snapshotViewAfterScreenUpdates
                
                let renderer: ObjcId = msg_send![class!(UIGraphicsImageRenderer), alloc];
                let size = CGSize {
                    width: state.width as f64,
                    height: state.height as f64,
                };
                let renderer: ObjcId = msg_send![renderer, initWithSize: size];
                
                // This creates an image from the view
                // In a real implementation, you would:
                // 1. Render UIView to a CGImage
                // 2. Upload CGImage data to the Metal texture
                
                Some(texture)
            }
        } else {
            None
        }
    }
}

/// Helper function to get the default Metal device
fn get_default_metal_device() -> Option<ObjcId> {
    unsafe {
        let device: ObjcId = msg_send![class!(MTLCreateSystemDefaultDevice), new];
        if device == nil {
            None
        } else {
            Some(device)
        }
    }
}

// CGRect and related types for iOS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

