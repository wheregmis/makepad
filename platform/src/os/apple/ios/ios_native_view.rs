// Minimal stub implementation for iOS native views
// Full implementation can be provided by makepad-native-components crate

use {
    std::collections::HashMap,
    crate::{
        makepad_math::*,
        native_view::{
            NativeViewId,
            NativeViewConfig,
            NativeViewEvent,
            NativeViewTouchEvent,
            NativeViewHandle,
        },
    },
};

/// Manager for iOS native views (stub)
pub struct IosNativeViewManager {
    pub views: HashMap<NativeViewId, ()>,
    pub pending_events: Vec<NativeViewEvent>,
}

impl Default for IosNativeViewManager {
    fn default() -> Self {
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
        }
    }
}

impl IosNativeViewManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn create_view(&mut self, _id: NativeViewId, _config: &NativeViewConfig) -> bool {
        false // Stub - returns false
    }
    
    pub fn update_view(&mut self, _id: NativeViewId, _config: &NativeViewConfig) -> bool {
        false // Stub
    }
    
    pub fn destroy_view(&mut self, _id: NativeViewId) -> bool {
        self.views.remove(&_id);
        false // Stub
    }
    
    pub fn set_frame(&mut self, _id: NativeViewId, _frame: Rect) -> bool {
        false // Stub
    }
    
    pub fn send_touch(&mut self, _event: NativeViewTouchEvent) -> bool {
        false // Stub
    }
    
    pub fn get_texture(&self, _id: NativeViewId) -> Option<&NativeViewHandle> {
        None // Stub
    }
    
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
