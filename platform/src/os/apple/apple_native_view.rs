// Shared Apple native view implementation for iOS and macOS
// Full implementation provided by makepad-native-components crate via trait

use {
    std::collections::HashMap,
    crate::{
        makepad_math::*,
        makepad_objc_sys::runtime::ObjcId,
        native_view::{
            NativeViewId,
            NativeViewConfig,
            NativeViewEvent,
            NativeViewTouchEvent,
            NativeViewHandle,
        },
    },
};

/// Trait for Apple native view implementation (iOS/macOS)
/// Implemented by makepad-native-components crate
pub trait AppleNativeViewImpl: Send + Sync {
    fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool;
    fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool;
    fn destroy_view(&mut self, id: NativeViewId) -> bool;
    fn set_frame(&mut self, id: NativeViewId, frame: Rect, clip: Rect) -> bool;
    fn send_touch(&mut self, event: NativeViewTouchEvent) -> bool;
    fn get_texture(&self, id: NativeViewId) -> Option<&NativeViewHandle>;
    fn poll_events(&mut self) -> Vec<NativeViewEvent>;
    
    /// Set content view (macOS-specific, optional for iOS)
    fn set_content_view(&mut self, _view: ObjcId, _dpi: f64) {
        // Default implementation does nothing (for iOS)
    }
}

/// Manager for Apple native views (delegates to trait implementation)
pub struct AppleNativeViewManager {
    pub views: HashMap<NativeViewId, ()>,
    pub pending_events: Vec<NativeViewEvent>,
    pub content_view: ObjcId,
    pub dpi_factor: f64,
    /// Optional implementation provided by crate
    pub impl_: Option<Box<dyn AppleNativeViewImpl>>,
}

impl Default for AppleNativeViewManager {
    fn default() -> Self {
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
            content_view: crate::makepad_objc_sys::runtime::nil,
            dpi_factor: 1.0,
            impl_: None,
        }
    }
}

impl AppleNativeViewManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the implementation provided by crate
    pub fn set_impl(&mut self, impl_: Box<dyn AppleNativeViewImpl>) {
        self.impl_ = Some(impl_);
    }
    
    pub fn set_content_view(&mut self, view: ObjcId, dpi: f64) {
        self.content_view = view;
        self.dpi_factor = dpi;
        if let Some(impl_) = &mut self.impl_ {
            impl_.set_content_view(view, dpi);
        }
    }
    
    pub fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        if let Some(impl_) = &mut self.impl_ {
            impl_.create_view(id, config)
        } else {
            false
        }
    }
    
    pub fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        if let Some(impl_) = &mut self.impl_ {
            impl_.update_view(id, config)
        } else {
            false
        }
    }
    
    pub fn destroy_view(&mut self, id: NativeViewId) -> bool {
        self.views.remove(&id);
        if let Some(impl_) = &mut self.impl_ {
            impl_.destroy_view(id)
        } else {
            false
        }
    }
    
    pub fn set_frame(&mut self, id: NativeViewId, frame: Rect, clip: Rect) -> bool {
        if let Some(impl_) = &mut self.impl_ {
            impl_.set_frame(id, frame, clip)
        } else {
            false
        }
    }
    
    pub fn send_touch(&mut self, event: NativeViewTouchEvent) -> bool {
        if let Some(impl_) = &mut self.impl_ {
            impl_.send_touch(event)
        } else {
            false
        }
    }
    
    pub fn get_texture(&self, id: NativeViewId) -> Option<&NativeViewHandle> {
        if let Some(impl_) = &self.impl_ {
            impl_.get_texture(id)
        } else {
            None
        }
    }
    
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        if let Some(impl_) = &mut self.impl_ {
            impl_.poll_events()
        } else {
            std::mem::take(&mut self.pending_events)
        }
    }
}

/// Macro to implement native view API for CxOs (shared between iOS and macOS)
#[macro_export]
macro_rules! impl_apple_native_view_api {
    () => {
        /// Register native view implementation from crate
        pub fn set_native_view_impl(&mut self, impl_: Box<dyn crate::os::apple::apple_native_view::AppleNativeViewImpl>) {
            self.native_view_manager.set_impl(impl_);
        }
        
        pub fn create_native_view(&mut self, id: crate::native_view::NativeViewId, config: &crate::native_view::NativeViewConfig) -> bool {
            self.native_view_manager.create_view(id, config)
        }
        
        pub fn update_native_view(&mut self, id: crate::native_view::NativeViewId, config: &crate::native_view::NativeViewConfig) -> bool {
            self.native_view_manager.update_view(id, config)
        }
        
        pub fn destroy_native_view(&mut self, id: crate::native_view::NativeViewId) -> bool {
            self.native_view_manager.destroy_view(id)
        }
        
        pub fn set_native_view_frame(&mut self, id: crate::native_view::NativeViewId, frame: crate::makepad_math::Rect, clip: crate::makepad_math::Rect) -> bool {
            self.native_view_manager.set_frame(id, frame, clip)
        }
        
        pub fn send_touch_to_native_view(&mut self, event: crate::native_view::NativeViewTouchEvent) -> bool {
            self.native_view_manager.send_touch(event)
        }
        
        pub fn get_native_view_texture(&self, id: crate::native_view::NativeViewId) -> Option<&crate::native_view::NativeViewHandle> {
            self.native_view_manager.get_texture(id)
        }
        
        pub fn poll_native_view_events(&mut self) -> Vec<crate::native_view::NativeViewEvent> {
            self.native_view_manager.poll_events()
        }
    };
}
