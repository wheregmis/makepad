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
        texture::Texture,
        os::apple::apple_sys::*,
        os::apple::apple_util::*,
    },
};

/// macOS-specific native view state
pub struct MacosNativeViewState {
    /// The native NSView
    pub view: ObjcId,
    /// Off-screen render target (CAMetalLayer)
    pub metal_layer: ObjcId,
    /// Current frame
    pub frame: Rect,
    /// Texture for rendering
    pub texture: Option<Texture>,
    /// Width in pixels
    pub width: usize,
    /// Height: in pixels
    pub height: usize,
    /// Scale factor
    pub scale: f64,
}

/// Manager for macOS native views
pub struct MacosNativeViewManager {
    pub views: HashMap<NativeViewId, MacosNativeViewState>,
    pub pending_events: Vec<NativeViewEvent>,
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
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
            content_view: nil,
            dpi_factor: 1.0,
        }
    }
    
    /// Set the content view where native views will be added
    pub fn set_content_view(&mut self, view: ObjcId, dpi_factor: f64) {
        self.content_view = view;
        self.dpi_factor = dpi_factor;
    }
    
    /// Create a native NSView based on the config
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
                    // Create a generic NSView for custom types
                    let view: ObjcId = msg_send![class!(NSView), alloc];
                    let view: ObjcId = msg_send![view, init];
                    view
                }
            };
            
            if view == nil {
                return false;
            }
            
            // Set up off-screen rendering with CAMetalLayer
            let metal_layer = self.setup_metal_layer(view, &config.frame, config.texture_scale);
            
            // Apply frame - note: we'll set the proper frame when added to content view
            // since we need the parent's height to flip Y coordinates
            let frame = config.frame;
            let ns_rect = NSRect {
                origin: NSPoint { x: frame.pos.x, y: frame.pos.y },
                size: NSSize { width: frame.size.x, height: frame.size.y },
            };
            let () = msg_send![view, setFrame: ns_rect];
            
            // Apply background color if specified  
            if let Some(color) = config.background_color {
                let () = msg_send![view, setWantsLayer: YES];
                let layer: ObjcId = msg_send![view, layer];
                if layer != nil {
                    let cg_color = create_cg_color(color.x as f64, color.y as f64, color.z as f64, color.w as f64);
                    let () = msg_send![layer, setBackgroundColor: cg_color];
                }
            }
            
            // Calculate pixel dimensions
            let scale = config.texture_scale.max(1.0);
            let width = (frame.size.x * scale) as usize;
            let height = (frame.size.y * scale) as usize;
            
            // Add to content view if available
            if self.content_view != nil {
                // Get parent bounds to flip Y coordinate (macOS uses bottom-left origin)
                let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                // Flip Y coordinate: macOS uses bottom-left, Makepad uses top-left
                let flipped_y = parent_bounds.size.height - frame.pos.y - frame.size.y;
                
                // Use exact frame from Makepad layout
                let ns_rect = NSRect {
                    origin: NSPoint { 
                        x: frame.pos.x, 
                        y: flipped_y 
                    },
                    size: NSSize { 
                        width: frame.size.x, 
                        height: frame.size.y 
                    },
                };
                let () = msg_send![view, setFrame: ns_rect];
                
                // Ensure the view is properly aligned
                let () = msg_send![view, setAutoresizingMask: 0u64]; // No autoresizing - we control it
                let () = msg_send![self.content_view, addSubview: view];
            }
            
            let state = MacosNativeViewState {
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
    
    /// Create an NSButton
    unsafe fn create_button(&self, label: &str) -> ObjcId {
        let button: ObjcId = msg_send![class!(NSButton), alloc];
        let button: ObjcId = msg_send![button, init];
        
        let ns_label = str_to_nsstring(label);
        let () = msg_send![button, setTitle: ns_label];
        let () = msg_send![button, setBezelStyle: 1i64]; // NSBezelStyleRounded
        
        button
    }
    
    /// Create an NSTextField
    unsafe fn create_text_field(&self, placeholder: &str, text: &str) -> ObjcId {
        let text_field: ObjcId = msg_send![class!(NSTextField), alloc];
        let text_field: ObjcId = msg_send![text_field, init];
        
        let ns_placeholder = str_to_nsstring(placeholder);
        let ns_text = str_to_nsstring(text);
        
        let () = msg_send![text_field, setPlaceholderString: ns_placeholder];
        let () = msg_send![text_field, setStringValue: ns_text];
        let () = msg_send![text_field, setBezeled: YES];
        let () = msg_send![text_field, setBezelStyle: 0i64]; // NSTextFieldSquareBezel
        
        text_field
    }
    
    /// Create an NSTextField configured as a label
    unsafe fn create_label(&self, text: &str) -> ObjcId {
        let label: ObjcId = msg_send![class!(NSTextField), alloc];
        let label: ObjcId = msg_send![label, init];
        
        let ns_text = str_to_nsstring(text);
        let () = msg_send![label, setStringValue: ns_text];
        let () = msg_send![label, setBezeled: NO];
        let () = msg_send![label, setDrawsBackground: NO];
        let () = msg_send![label, setEditable: NO];
        let () = msg_send![label, setSelectable: NO];
        
        label
    }
    
    /// Create an NSSwitch (macOS 10.15+) or NSButton with switch style
    unsafe fn create_switch(&self, on: bool) -> ObjcId {
        // NSSwitch is available on macOS 10.15+
        // Fall back to checkbox-style button for older versions
        let switch: ObjcId = msg_send![class!(NSSwitch), alloc];
        if switch != nil {
            let switch: ObjcId = msg_send![switch, init];
            let state: i64 = if on { 1 } else { 0 };
            let () = msg_send![switch, setState: state];
            return switch;
        }
        
        // Fallback: use NSButton with switch style
        let button: ObjcId = msg_send![class!(NSButton), alloc];
        let button: ObjcId = msg_send![button, init];
        let () = msg_send![button, setButtonType: 3i64]; // NSSwitchButton
        let state: i64 = if on { 1 } else { 0 };
        let () = msg_send![button, setState: state];
        
        button
    }
    
    /// Create an NSSlider
    unsafe fn create_slider(&self, value: f64, min: f64, max: f64) -> ObjcId {
        let slider: ObjcId = msg_send![class!(NSSlider), alloc];
        let slider: ObjcId = msg_send![slider, init];
        
        let () = msg_send![slider, setMinValue: min];
        let () = msg_send![slider, setMaxValue: max];
        let () = msg_send![slider, setDoubleValue: value];
        
        slider
    }
    
    /// Create an NSProgressIndicator
    unsafe fn create_progress_indicator(&self, progress: f64) -> ObjcId {
        let progress_indicator: ObjcId = msg_send![class!(NSProgressIndicator), alloc];
        let progress_indicator: ObjcId = msg_send![progress_indicator, init];
        
        let () = msg_send![progress_indicator, setStyle: 0i64]; // NSProgressIndicatorStyleBar
        let () = msg_send![progress_indicator, setIndeterminate: NO];
        let () = msg_send![progress_indicator, setMinValue: 0.0f64];
        let () = msg_send![progress_indicator, setMaxValue: 1.0f64];
        let () = msg_send![progress_indicator, setDoubleValue: progress];
        
        progress_indicator
    }
    
    /// Set up a CAMetalLayer for off-screen rendering
    unsafe fn setup_metal_layer(&self, view: ObjcId, frame: &Rect, scale: f64) -> ObjcId {
        // Enable layer backing
        let () = msg_send![view, setWantsLayer: YES];
        
        // Create a CAMetalLayer
        let metal_layer: ObjcId = msg_send![class!(CAMetalLayer), layer];
        
        // Get the Metal device
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
        
        // Set the layer on the view
        let () = msg_send![view, setLayer: metal_layer];
        
        metal_layer
    }
    
    /// Update an existing native view
    pub fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            unsafe {
                // Update frame with proper Y coordinate flipping
                let frame = config.frame;
                let flipped_y = if self.content_view != nil {
                    let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                    parent_bounds.size.height - frame.pos.y - frame.size.y
                } else {
                    frame.pos.y
                };
                
                let ns_rect = NSRect {
                    origin: NSPoint { x: frame.pos.x, y: flipped_y },
                    size: NSSize { width: frame.size.x, height: frame.size.y },
                };
                let () = msg_send![state.view, setFrame: ns_rect];
                
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
                        let () = msg_send![state.view, setTitle: ns_label];
                    }
                    NativeViewType::TextField { placeholder, text } => {
                        let ns_placeholder = str_to_nsstring(placeholder);
                        let ns_text = str_to_nsstring(text);
                        let () = msg_send![state.view, setPlaceholderString: ns_placeholder];
                        let () = msg_send![state.view, setStringValue: ns_text];
                    }
                    NativeViewType::Label { text } => {
                        let ns_text = str_to_nsstring(text);
                        let () = msg_send![state.view, setStringValue: ns_text];
                    }
                    NativeViewType::Switch { on } => {
                        let state_val: i64 = if *on { 1 } else { 0 };
                        let () = msg_send![state.view, setState: state_val];
                    }
                    NativeViewType::Slider { value, min, max } => {
                        let () = msg_send![state.view, setMinValue: *min];
                        let () = msg_send![state.view, setMaxValue: *max];
                        let () = msg_send![state.view, setDoubleValue: *value];
                    }
                    NativeViewType::ProgressIndicator { progress } => {
                        let () = msg_send![state.view, setDoubleValue: *progress];
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
                // Flip Y coordinate for macOS coordinate system
                let flipped_y = if self.content_view != nil {
                    let parent_bounds: NSRect = msg_send![self.content_view, bounds];
                    parent_bounds.size.height - frame.pos.y - frame.size.y
                } else {
                    frame.pos.y
                };
                
                // Use exact frame from Makepad - no rounding or adjustment
                let ns_rect = NSRect {
                    origin: NSPoint { 
                        x: frame.pos.x, 
                        y: flipped_y 
                    },
                    size: NSSize { 
                        width: frame.size.x, 
                        height: frame.size.y 
                    },
                };
                let () = msg_send![state.view, setFrame: ns_rect];
                
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
    
    /// Forward a mouse/touch event to a native view
    pub fn send_touch(&mut self, event: NativeViewTouchEvent) -> bool {
        if let Some(state) = self.views.get(&event.id) {
            unsafe {
                // For buttons, simulate click on touch end
                let class_name: ObjcId = msg_send![state.view, className];
                let class_str = nsstring_to_string(class_name);
                
                if class_str == "NSButton" {
                    match event.phase {
                        NativeViewTouchPhase::Ended => {
                            let () = msg_send![state.view, performClick: nil];
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
    pub fn get_texture(&self, _id: NativeViewId) -> Option<&NativeViewHandle> {
        None // Texture management is deferred
    }
    
    /// Poll for events from native views
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

/// Helper function to get the default Metal device
fn get_default_metal_device() -> Option<ObjcId> {
    unsafe {
        // MTLCreateSystemDefaultDevice
        #[link(name = "Metal", kind = "framework")]
        extern "C" {
            fn MTLCreateSystemDefaultDevice() -> ObjcId;
        }
        let device = MTLCreateSystemDefaultDevice();
        if device == nil {
            None
        } else {
            Some(device)
        }
    }
}

/// Helper to create a CGColor
unsafe fn create_cg_color(r: f64, g: f64, b: f64, a: f64) -> ObjcId {
    let color_space: ObjcId = msg_send![class!(NSColorSpace), deviceRGBColorSpace];
    let ns_color: ObjcId = msg_send![
        class!(NSColor),
        colorWithColorSpace: color_space
        components: [r, g, b, a].as_ptr()
        count: 4usize
    ];
    msg_send![ns_color, CGColor]
}

// CGRect and related types for macOS
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

