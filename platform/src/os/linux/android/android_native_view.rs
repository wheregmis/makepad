use {
    std::collections::HashMap,
    makepad_jni_sys as jni_sys,
    crate::{
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
    },
    super::android_jni::{attach_jni_env, to_java_string},
};

/// Android-specific native view state
pub struct AndroidNativeViewState {
    /// The native Android View (jobject reference)
    pub view: jni_sys::jobject,
    /// SurfaceTexture for texture sharing
    pub surface_texture: Option<jni_sys::jobject>,
    /// OpenGL texture ID
    pub texture_id: u32,
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

/// Manager for Android native views
#[derive(Default)]
pub struct AndroidNativeViewManager {
    pub views: HashMap<NativeViewId, AndroidNativeViewState>,
    pub pending_events: Vec<NativeViewEvent>,
}

impl AndroidNativeViewManager {
    pub fn new() -> Self {
        Self {
            views: HashMap::new(),
            pending_events: Vec::new(),
        }
    }
    
    /// Create a native Android View based on the config
    pub fn create_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        let env = attach_jni_env();
        
        unsafe {
            let view: jni_sys::jobject = match &config.view_type {
                NativeViewType::Button { label } => {
                    self.create_button(env, label)
                }
                NativeViewType::TextField { placeholder, text } => {
                    self.create_text_field(env, placeholder, text)
                }
                NativeViewType::Label { text } => {
                    self.create_label(env, text)
                }
                NativeViewType::Switch { on } => {
                    self.create_switch(env, *on)
                }
                NativeViewType::Slider { value, min, max } => {
                    self.create_slider(env, *value, *min, *max)
                }
                NativeViewType::ProgressIndicator { progress } => {
                    self.create_progress_indicator(env, *progress)
                }
                NativeViewType::Custom { type_name, .. } => {
                    self.create_custom_view(env, type_name)
                }
            };
            
            if view.is_null() {
                return false;
            }
            
            // Create a global reference to prevent garbage collection
            let global_ref = (**env).NewGlobalRef.unwrap()(env, view);
            
            // Set up SurfaceTexture for texture sharing
            let (surface_texture, texture_id) = self.setup_surface_texture(env);
            
            // Calculate pixel dimensions
            let scale = config.texture_scale.max(1.0);
            let width = (config.frame.size.x * scale) as usize;
            let height = (config.frame.size.y * scale) as usize;
            
            let state = AndroidNativeViewState {
                view: global_ref,
                surface_texture,
                texture_id,
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
    
    /// Create a Button (android.widget.Button)
    unsafe fn create_button(&self, env: *mut jni_sys::JNIEnv, label: &str) -> jni_sys::jobject {
        // Get the activity context
        let activity = makepad_android_state::get_activity();
        
        // Find Button class
        let button_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/Button\0".as_ptr() as *const i8
        );
        
        if button_class.is_null() {
            return std::ptr::null_mut();
        }
        
        // Get constructor
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            button_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        // Create button
        let button = (**env).NewObject.unwrap()(env, button_class, constructor, activity);
        
        if !button.is_null() {
            // Set text
            let set_text = (**env).GetMethodID.unwrap()(
                env,
                button_class,
                b"setText\0".as_ptr() as *const i8,
                b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
            );
            
            let label_jstring = to_java_string(env, label);
            (**env).CallVoidMethod.unwrap()(env, button, set_text, label_jstring);
            (**env).DeleteLocalRef.unwrap()(env, label_jstring);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, button_class);
        button
    }
    
    /// Create an EditText (android.widget.EditText)
    unsafe fn create_text_field(&self, env: *mut jni_sys::JNIEnv, placeholder: &str, text: &str) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        let edit_text_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/EditText\0".as_ptr() as *const i8
        );
        
        if edit_text_class.is_null() {
            return std::ptr::null_mut();
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            edit_text_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        let edit_text = (**env).NewObject.unwrap()(env, edit_text_class, constructor, activity);
        
        if !edit_text.is_null() {
            // Set hint (placeholder)
            let set_hint = (**env).GetMethodID.unwrap()(
                env,
                edit_text_class,
                b"setHint\0".as_ptr() as *const i8,
                b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
            );
            let hint_jstring = to_java_string(env, placeholder);
            (**env).CallVoidMethod.unwrap()(env, edit_text, set_hint, hint_jstring);
            (**env).DeleteLocalRef.unwrap()(env, hint_jstring);
            
            // Set text
            let set_text = (**env).GetMethodID.unwrap()(
                env,
                edit_text_class,
                b"setText\0".as_ptr() as *const i8,
                b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
            );
            let text_jstring = to_java_string(env, text);
            (**env).CallVoidMethod.unwrap()(env, edit_text, set_text, text_jstring);
            (**env).DeleteLocalRef.unwrap()(env, text_jstring);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, edit_text_class);
        edit_text
    }
    
    /// Create a TextView (android.widget.TextView)
    unsafe fn create_label(&self, env: *mut jni_sys::JNIEnv, text: &str) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        let text_view_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/TextView\0".as_ptr() as *const i8
        );
        
        if text_view_class.is_null() {
            return std::ptr::null_mut();
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            text_view_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        let text_view = (**env).NewObject.unwrap()(env, text_view_class, constructor, activity);
        
        if !text_view.is_null() {
            let set_text = (**env).GetMethodID.unwrap()(
                env,
                text_view_class,
                b"setText\0".as_ptr() as *const i8,
                b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
            );
            let text_jstring = to_java_string(env, text);
            (**env).CallVoidMethod.unwrap()(env, text_view, set_text, text_jstring);
            (**env).DeleteLocalRef.unwrap()(env, text_jstring);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, text_view_class);
        text_view
    }
    
    /// Create a Switch (android.widget.Switch)
    unsafe fn create_switch(&self, env: *mut jni_sys::JNIEnv, on: bool) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        let switch_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/Switch\0".as_ptr() as *const i8
        );
        
        if switch_class.is_null() {
            return std::ptr::null_mut();
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            switch_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        let switch = (**env).NewObject.unwrap()(env, switch_class, constructor, activity);
        
        if !switch.is_null() {
            let set_checked = (**env).GetMethodID.unwrap()(
                env,
                switch_class,
                b"setChecked\0".as_ptr() as *const i8,
                b"(Z)V\0".as_ptr() as *const i8
            );
            (**env).CallVoidMethod.unwrap()(env, switch, set_checked, on as jni_sys::jboolean);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, switch_class);
        switch
    }
    
    /// Create a SeekBar (android.widget.SeekBar)
    unsafe fn create_slider(&self, env: *mut jni_sys::JNIEnv, value: f64, min: f64, max: f64) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        let seekbar_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/SeekBar\0".as_ptr() as *const i8
        );
        
        if seekbar_class.is_null() {
            return std::ptr::null_mut();
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            seekbar_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        let seekbar = (**env).NewObject.unwrap()(env, seekbar_class, constructor, activity);
        
        if !seekbar.is_null() {
            // Set max (SeekBar uses integer values, so we scale)
            let set_max = (**env).GetMethodID.unwrap()(
                env,
                seekbar_class,
                b"setMax\0".as_ptr() as *const i8,
                b"(I)V\0".as_ptr() as *const i8
            );
            let range = (max - min) as i32;
            (**env).CallVoidMethod.unwrap()(env, seekbar, set_max, range.max(100));
            
            // Set progress
            let set_progress = (**env).GetMethodID.unwrap()(
                env,
                seekbar_class,
                b"setProgress\0".as_ptr() as *const i8,
                b"(I)V\0".as_ptr() as *const i8
            );
            let progress = ((value - min) / (max - min) * range as f64) as i32;
            (**env).CallVoidMethod.unwrap()(env, seekbar, set_progress, progress);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, seekbar_class);
        seekbar
    }
    
    /// Create a ProgressBar (android.widget.ProgressBar)
    unsafe fn create_progress_indicator(&self, env: *mut jni_sys::JNIEnv, progress: f64) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        let progress_bar_class = (**env).FindClass.unwrap()(
            env,
            b"android/widget/ProgressBar\0".as_ptr() as *const i8
        );
        
        if progress_bar_class.is_null() {
            return std::ptr::null_mut();
        }
        
        // Use horizontal style constructor
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            progress_bar_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;Landroid/util/AttributeSet;I)V\0".as_ptr() as *const i8
        );
        
        // android.R.attr.progressBarStyleHorizontal = 16842872
        let progress_bar = (**env).NewObject.unwrap()(
            env, 
            progress_bar_class, 
            constructor, 
            activity,
            std::ptr::null_mut::<jni_sys::_jobject>(),
            16842872i32
        );
        
        if !progress_bar.is_null() {
            let set_max = (**env).GetMethodID.unwrap()(
                env,
                progress_bar_class,
                b"setMax\0".as_ptr() as *const i8,
                b"(I)V\0".as_ptr() as *const i8
            );
            (**env).CallVoidMethod.unwrap()(env, progress_bar, set_max, 100i32);
            
            let set_progress = (**env).GetMethodID.unwrap()(
                env,
                progress_bar_class,
                b"setProgress\0".as_ptr() as *const i8,
                b"(I)V\0".as_ptr() as *const i8
            );
            (**env).CallVoidMethod.unwrap()(env, progress_bar, set_progress, (progress * 100.0) as i32);
        }
        
        (**env).DeleteLocalRef.unwrap()(env, progress_bar_class);
        progress_bar
    }
    
    /// Create a custom View
    unsafe fn create_custom_view(&self, env: *mut jni_sys::JNIEnv, type_name: &str) -> jni_sys::jobject {
        let activity = makepad_android_state::get_activity();
        
        // Try to find the class by name
        let class_name = type_name.replace('.', "/");
        let class_name_cstr = std::ffi::CString::new(class_name).unwrap();
        
        let view_class = (**env).FindClass.unwrap()(env, class_name_cstr.as_ptr());
        
        if view_class.is_null() {
            // Fall back to generic View
            let view_class = (**env).FindClass.unwrap()(
                env,
                b"android/view/View\0".as_ptr() as *const i8
            );
            
            if view_class.is_null() {
                return std::ptr::null_mut();
            }
            
            let constructor = (**env).GetMethodID.unwrap()(
                env,
                view_class,
                b"<init>\0".as_ptr() as *const i8,
                b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
            );
            
            let view = (**env).NewObject.unwrap()(env, view_class, constructor, activity);
            (**env).DeleteLocalRef.unwrap()(env, view_class);
            return view;
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            view_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(Landroid/content/Context;)V\0".as_ptr() as *const i8
        );
        
        let view = (**env).NewObject.unwrap()(env, view_class, constructor, activity);
        (**env).DeleteLocalRef.unwrap()(env, view_class);
        view
    }
    
    /// Set up SurfaceTexture for texture sharing
    unsafe fn setup_surface_texture(&self, env: *mut jni_sys::JNIEnv) -> (Option<jni_sys::jobject>, u32) {
        // Generate an OpenGL texture
        let mut texture_id: u32 = 0;
        // Note: GL calls would need to be made on the render thread
        // For now, return 0 and handle texture creation during rendering
        
        // Create SurfaceTexture
        let surface_texture_class = (**env).FindClass.unwrap()(
            env,
            b"android/graphics/SurfaceTexture\0".as_ptr() as *const i8
        );
        
        if surface_texture_class.is_null() {
            return (None, 0);
        }
        
        let constructor = (**env).GetMethodID.unwrap()(
            env,
            surface_texture_class,
            b"<init>\0".as_ptr() as *const i8,
            b"(I)V\0".as_ptr() as *const i8
        );
        
        // Use texture ID 0 for now, will be updated when GL context is available
        let surface_texture = (**env).NewObject.unwrap()(
            env, 
            surface_texture_class, 
            constructor, 
            texture_id as jni_sys::jint
        );
        
        if surface_texture.is_null() {
            (**env).DeleteLocalRef.unwrap()(env, surface_texture_class);
            return (None, 0);
        }
        
        let global_ref = (**env).NewGlobalRef.unwrap()(env, surface_texture);
        (**env).DeleteLocalRef.unwrap()(env, surface_texture);
        (**env).DeleteLocalRef.unwrap()(env, surface_texture_class);
        
        (Some(global_ref), texture_id)
    }
    
    /// Update an existing native view
    pub fn update_view(&mut self, id: NativeViewId, config: &NativeViewConfig) -> bool {
        let env = attach_jni_env();
        
        if let Some(state) = self.views.get_mut(&id) {
            unsafe {
                // Update view-specific properties
                match &config.view_type {
                    NativeViewType::Button { label } => {
                        let button_class = (**env).GetObjectClass.unwrap()(env, state.view);
                        let set_text = (**env).GetMethodID.unwrap()(
                            env,
                            button_class,
                            b"setText\0".as_ptr() as *const i8,
                            b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
                        );
                        let label_jstring = to_java_string(env, label);
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_text, label_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, label_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, button_class);
                    }
                    NativeViewType::TextField { placeholder, text } => {
                        let class = (**env).GetObjectClass.unwrap()(env, state.view);
                        
                        let set_hint = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setHint\0".as_ptr() as *const i8,
                            b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
                        );
                        let hint_jstring = to_java_string(env, placeholder);
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_hint, hint_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, hint_jstring);
                        
                        let set_text = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setText\0".as_ptr() as *const i8,
                            b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
                        );
                        let text_jstring = to_java_string(env, text);
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_text, text_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, text_jstring);
                        
                        (**env).DeleteLocalRef.unwrap()(env, class);
                    }
                    NativeViewType::Label { text } => {
                        let class = (**env).GetObjectClass.unwrap()(env, state.view);
                        let set_text = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setText\0".as_ptr() as *const i8,
                            b"(Ljava/lang/CharSequence;)V\0".as_ptr() as *const i8
                        );
                        let text_jstring = to_java_string(env, text);
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_text, text_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, text_jstring);
                        (**env).DeleteLocalRef.unwrap()(env, class);
                    }
                    NativeViewType::Switch { on } => {
                        let class = (**env).GetObjectClass.unwrap()(env, state.view);
                        let set_checked = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setChecked\0".as_ptr() as *const i8,
                            b"(Z)V\0".as_ptr() as *const i8
                        );
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_checked, *on as jni_sys::jboolean);
                        (**env).DeleteLocalRef.unwrap()(env, class);
                    }
                    NativeViewType::Slider { value, min, max } => {
                        let class = (**env).GetObjectClass.unwrap()(env, state.view);
                        let set_progress = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setProgress\0".as_ptr() as *const i8,
                            b"(I)V\0".as_ptr() as *const i8
                        );
                        let range = (max - min) as i32;
                        let progress = ((value - min) / (max - min) * range as f64) as i32;
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_progress, progress);
                        (**env).DeleteLocalRef.unwrap()(env, class);
                    }
                    NativeViewType::ProgressIndicator { progress } => {
                        let class = (**env).GetObjectClass.unwrap()(env, state.view);
                        let set_progress = (**env).GetMethodID.unwrap()(
                            env, class,
                            b"setProgress\0".as_ptr() as *const i8,
                            b"(I)V\0".as_ptr() as *const i8
                        );
                        (**env).CallVoidMethod.unwrap()(env, state.view, set_progress, (*progress * 100.0) as i32);
                        (**env).DeleteLocalRef.unwrap()(env, class);
                    }
                    NativeViewType::Custom { .. } => {}
                }
                
                // Update frame
                state.frame = config.frame;
                let scale = config.texture_scale.max(1.0);
                state.width = (config.frame.size.x * scale) as usize;
                state.height = (config.frame.size.y * scale) as usize;
                state.scale = scale;
            }
            true
        } else {
            false
        }
    }
    
    /// Destroy a native view
    pub fn destroy_view(&mut self, id: NativeViewId) -> bool {
        let env = attach_jni_env();
        
        if let Some(state) = self.views.remove(&id) {
            unsafe {
                // Delete the global reference
                (**env).DeleteGlobalRef.unwrap()(env, state.view);
                
                // Delete SurfaceTexture if present
                if let Some(surface_texture) = state.surface_texture {
                    (**env).DeleteGlobalRef.unwrap()(env, surface_texture);
                }
            }
            true
        } else {
            false
        }
    }
    
    /// Set the frame of a native view
    pub fn set_frame(&mut self, id: NativeViewId, frame: Rect) -> bool {
        if let Some(state) = self.views.get_mut(&id) {
            state.frame = frame;
            state.width = (frame.size.x * state.scale) as usize;
            state.height = (frame.size.y * state.scale) as usize;
            // Note: Actual layout update would need to happen on the UI thread
            true
        } else {
            false
        }
    }
    
    /// Forward a touch event to a native view
    pub fn send_touch(&mut self, event: NativeViewTouchEvent) -> bool {
        if let Some(state) = self.views.get(&event.id) {
            let env = attach_jni_env();
            
            unsafe {
                // For buttons, simulate click on touch end
                let class = (**env).GetObjectClass.unwrap()(env, state.view);
                let class_name: jni_sys::jobject = (**env).CallObjectMethod.unwrap()(
                    env,
                    class,
                    (**env).GetMethodID.unwrap()(
                        env,
                        class,
                        b"getName\0".as_ptr() as *const i8,
                        b"()Ljava/lang/String;\0".as_ptr() as *const i8
                    )
                );
                
                // Check if it's a button and handle click
                if let NativeViewTouchPhase::Ended = event.phase {
                    let perform_click = (**env).GetMethodID.unwrap()(
                        env,
                        class,
                        b"performClick\0".as_ptr() as *const i8,
                        b"()Z\0".as_ptr() as *const i8
                    );
                    
                    if !perform_click.is_null() {
                        (**env).CallBooleanMethod.unwrap()(env, state.view, perform_click);
                        self.pending_events.push(NativeViewEvent::ButtonTapped { id: event.id });
                    }
                }
                
                if !class_name.is_null() {
                    (**env).DeleteLocalRef.unwrap()(env, class_name);
                }
                (**env).DeleteLocalRef.unwrap()(env, class);
            }
            true
        } else {
            false
        }
    }
    
    /// Get the texture handle for a native view
    pub fn get_texture(&self, id: NativeViewId) -> Option<&NativeViewHandle> {
        None // Texture management is deferred
    }
    
    /// Poll for events from native views
    pub fn poll_events(&mut self) -> Vec<NativeViewEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

