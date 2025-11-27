// macOS native view - uses shared Apple implementation
pub use crate::os::apple::apple_native_view::{AppleNativeViewImpl, AppleNativeViewManager};

// Type alias for macOS (manager only, trait can't be aliased)
pub type MacosNativeViewManager = crate::os::apple::apple_native_view::AppleNativeViewManager;
