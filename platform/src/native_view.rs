use {
    crate::{
        makepad_live_id::*,
        makepad_math::*,
        texture::Texture,
        cx::Cx,
    },
    std::collections::HashMap,
};

/// Unique identifier for a native view instance
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, FromLiveId)]
pub struct NativeViewId(pub LiveId);

/// Types of native views that can be embedded
#[derive(Clone, Debug)]
pub enum NativeViewType {
    /// A native button
    Button { label: String },
    /// A native text field
    TextField { placeholder: String, text: String },
    /// A native label
    Label { text: String },
    /// A native switch/toggle
    Switch { on: bool },
    /// A native slider
    Slider { value: f64, min: f64, max: f64 },
    /// A native progress indicator
    ProgressIndicator { progress: f64 },
    /// A custom native view (platform-specific identifier)
    Custom { type_name: String, properties: HashMap<String, String> },
}

/// Configuration for creating a native view
#[derive(Clone, Debug)]
pub struct NativeViewConfig {
    /// The type of native view to create
    pub view_type: NativeViewType,
    /// Initial frame/bounds in logical pixels
    pub frame: Rect,
    /// Background color (if supported)
    pub background_color: Option<Vec4>,
    /// Whether the view is interactive
    pub interactive: bool,
    /// Scale factor for the texture (1.0 = native resolution)
    pub texture_scale: f64,
}

impl Default for NativeViewConfig {
    fn default() -> Self {
        Self {
            view_type: NativeViewType::Label { text: String::new() },
            frame: Rect::default(),
            background_color: None,
            interactive: true,
            texture_scale: 1.0,
        }
    }
}

/// Events that can be received from native views
#[derive(Clone, Debug)]
pub enum NativeViewEvent {
    /// Button was tapped
    ButtonTapped { id: NativeViewId },
    /// Text field content changed
    TextChanged { id: NativeViewId, text: String },
    /// Text field editing began
    TextEditingBegan { id: NativeViewId },
    /// Text field editing ended
    TextEditingEnded { id: NativeViewId },
    /// Switch value changed
    SwitchChanged { id: NativeViewId, on: bool },
    /// Slider value changed
    SliderChanged { id: NativeViewId, value: f64 },
    /// Generic interaction event
    Interaction { id: NativeViewId, action: String },
    /// Texture was updated and needs redraw
    TextureUpdated { id: NativeViewId },
}

/// Touch event to forward to native views
#[derive(Clone, Debug)]
pub struct NativeViewTouchEvent {
    pub id: NativeViewId,
    pub phase: NativeViewTouchPhase,
    pub position: DVec2,
    pub time: f64,
}

#[derive(Clone, Copy, Debug)]
pub enum NativeViewTouchPhase {
    Began,
    Moved,
    Ended,
    Cancelled,
}

/// Handle to a native view's texture
#[derive(Clone)]
pub struct NativeViewHandle {
    pub id: NativeViewId,
    pub texture: Texture,
    pub width: usize,
    pub height: usize,
}

/// Platform-specific native view manager trait
pub trait CxNativeViewApi {
    /// Create a new native view
    fn create_native_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool;
    
    /// Update an existing native view's configuration
    fn update_native_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool;
    
    /// Destroy a native view
    fn destroy_native_view(&mut self, id: NativeViewId) -> bool;
    
    /// Update the frame/position of a native view
    fn set_native_view_frame(&mut self, id: NativeViewId, frame: Rect) -> bool;
    
    /// Forward a touch event to a native view
    fn send_touch_to_native_view(&mut self, event: NativeViewTouchEvent) -> bool;
    
    /// Get the texture handle for a native view
    fn get_native_view_texture(&self, id: NativeViewId) -> Option<&NativeViewHandle>;
    
    /// Poll for events from native views
    fn poll_native_view_events(&mut self) -> Vec<NativeViewEvent>;
    
    /// Request texture update for a native view
    fn request_native_view_texture_update(&mut self, id: NativeViewId);
}

/// Storage for native views in Cx
#[derive(Default)]
pub struct CxNativeViews {
    pub views: HashMap<NativeViewId, NativeViewState>,
    pub pending_events: Vec<NativeViewEvent>,
}

/// State of a native view
pub struct NativeViewState {
    pub config: NativeViewConfig,
    pub handle: Option<NativeViewHandle>,
    pub needs_texture_update: bool,
}

impl Cx {
    /// Create a new native view
    pub fn create_native_view(&mut self, id: NativeViewId, config: NativeViewConfig) -> bool {
        // Store the configuration
        self.native_views.views.insert(id, NativeViewState {
            config: config.clone(),
            handle: None,
            needs_texture_update: true,
        });
        
        // Platform-specific creation will be handled by the OS layer
        #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
        {
            return self.os.create_native_view(id, &config);
        }
        
        #[cfg(target_os = "android")]
        {
            return self.os.create_native_view(id, &config);
        }
        
        #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "android")))]
        {
            // Unsupported platform
            false
        }
    }
    
    /// Update a native view's configuration
    pub fn update_native_view(&mut self, id: NativeViewId, config: NativeViewConfig) -> bool {
        if let Some(state) = self.native_views.views.get_mut(&id) {
            state.config = config.clone();
            state.needs_texture_update = true;
            
            #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
            {
                return self.os.update_native_view(id, &config);
            }
            
            #[cfg(target_os = "android")]
            {
                return self.os.update_native_view(id, &config);
            }
        }
        
        false
    }
    
    /// Destroy a native view
    pub fn destroy_native_view(&mut self, id: NativeViewId) -> bool {
        self.native_views.views.remove(&id);
        
        #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
        {
            return self.os.destroy_native_view(id);
        }
        
        #[cfg(target_os = "android")]
        {
            return self.os.destroy_native_view(id);
        }
        
        #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "android")))]
        {
            false
        }
    }
    
    /// Set the frame of a native view
    pub fn set_native_view_frame(&mut self, id: NativeViewId, frame: Rect) -> bool {
        if let Some(state) = self.native_views.views.get_mut(&id) {
            state.config.frame = frame;
            state.needs_texture_update = true;
            
            #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
            {
                return self.os.set_native_view_frame(id, frame);
            }
            
            #[cfg(target_os = "android")]
            {
                return self.os.set_native_view_frame(id, frame);
            }
        }
        
        false
    }
    
    /// Get the texture for a native view
    pub fn get_native_view_texture(&self, id: NativeViewId) -> Option<&Texture> {
        self.native_views.views.get(&id)
            .and_then(|state| state.handle.as_ref())
            .map(|handle| &handle.texture)
    }
    
    /// Poll events from native views
    pub fn poll_native_view_events(&mut self) -> Vec<NativeViewEvent> {
        #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
        {
            return self.os.poll_native_view_events();
        }
        
        #[cfg(target_os = "android")]
        {
            return self.os.poll_native_view_events();
        }
        
        #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "android")))]
        {
            std::mem::take(&mut self.native_views.pending_events)
        }
    }
    
    /// Forward touch event to native view
    pub fn send_touch_to_native_view(&mut self, event: NativeViewTouchEvent) -> bool {
        #[cfg(any(target_os = "ios", target_os = "macos", target_os = "tvos"))]
        {
            return self.os.send_touch_to_native_view(event);
        }
        
        #[cfg(target_os = "android")]
        {
            return self.os.send_touch_to_native_view(event);
        }
        
        #[cfg(not(any(target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "android")))]
        {
            let _ = event;
            false
        }
    }
}

