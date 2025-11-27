// iOS native view - uses shared Apple implementation
pub use crate::os::apple::apple_native_view::{AppleNativeViewImpl, AppleNativeViewManager};

// Type alias for iOS (manager only, trait can't be aliased)
pub type IosNativeViewManager = crate::os::apple::apple_native_view::AppleNativeViewManager;
