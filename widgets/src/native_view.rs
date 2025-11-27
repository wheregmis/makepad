use crate::{
    makepad_derive_widget::*,
    makepad_draw::*,
    widget::*,
};

live_design! {
    link widgets;
    use link::shaders::*;
    
    DrawNativeView = {{DrawNativeView}} {
        texture native_texture: texture2d
        
        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }
    
    pub NativeViewBase = {{NativeView}} {}
    
    pub NativeView = <NativeViewBase> {
        width: 200
        height: 100
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeView {
    #[deref] draw_super: DrawQuad,
}

/// Type of native view to create
#[derive(Clone, Debug, Live, LiveHook)]
#[live_ignore]
pub enum NativeViewKind {
    #[pick] Button,
    TextField,
    Label,
    Switch,
    Slider,
    ProgressIndicator,
    #[live(String::new())] Custom(String),
}

impl Default for NativeViewKind {
    fn default() -> Self {
        Self::Button
    }
}

/// A widget that embeds native platform UI components
#[derive(Live, Widget)]
pub struct NativeView {
    #[walk] walk: Walk,
    #[redraw] #[live] pub draw_bg: DrawNativeView,
    #[layout] layout: Layout,
    
    /// The type of native view to create
    #[live] pub view_kind: NativeViewKind,
    
    /// Label text for buttons
    #[live] pub label: String,
    
    /// Placeholder text for text fields
    #[live] pub placeholder: String,
    
    /// Current text value
    #[live] pub text: String,
    
    /// Switch/toggle state
    #[live] pub switch_on: bool,
    
    /// Slider value (0.0 to 1.0)
    #[live(0.5)] pub slider_value: f64,
    
    /// Slider minimum value
    #[live(0.0)] pub slider_min: f64,
    
    /// Slider maximum value
    #[live(1.0)] pub slider_max: f64,
    
    /// Progress value (0.0 to 1.0)
    #[live(0.0)] pub progress: f64,
    
    /// Scale factor for the native view texture
    #[live(2.0)] pub texture_scale: f64,
    
    /// Whether the native view is interactive
    #[live(true)] pub interactive: bool,
    
    /// Background color
    #[live] pub background_color: Option<Vec4>,
    
    /// Unique ID for the native view (auto-generated)
    #[rust] native_view_id: Option<NativeViewId>,
    
    /// The texture for rendering the native view
    #[rust] texture: Option<Texture>,
    
    /// Whether the native view has been created
    #[rust] is_created: bool,
    
    /// Last known rect for the widget
    #[rust] last_rect: Option<Rect>,
}

impl LiveHook for NativeView {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        // Update the native view when properties change
        if self.is_created {
            self.update_native_view(cx);
        }
    }
}

impl Widget for NativeView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        
        // Handle hits on the widget area
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                if let Some(id) = self.native_view_id {
                    if let Some(rect) = self.last_rect {
                        let local_pos = fe.abs - rect.pos;
                        cx.send_touch_to_native_view(NativeViewTouchEvent {
                            id,
                            phase: NativeViewTouchPhase::Began,
                            position: local_pos,
                            time: 0.0,
                        });
                    }
                }
            }
            Hit::FingerMove(fe) => {
                if let Some(id) = self.native_view_id {
                    if let Some(rect) = self.last_rect {
                        let local_pos = fe.abs - rect.pos;
                        cx.send_touch_to_native_view(NativeViewTouchEvent {
                            id,
                            phase: NativeViewTouchPhase::Moved,
                            position: local_pos,
                            time: 0.0,
                        });
                    }
                }
            }
            Hit::FingerUp(fe) => {
                if let Some(id) = self.native_view_id {
                    if let Some(rect) = self.last_rect {
                        let local_pos = fe.abs - rect.pos;
                        cx.send_touch_to_native_view(NativeViewTouchEvent {
                            id,
                            phase: NativeViewTouchPhase::Ended,
                            position: local_pos,
                            time: 0.0,
                        });
                    }
                }
            }
            _ => {}
        }
        
        // Poll for native view events
        let events = cx.poll_native_view_events();
        for native_event in events {
            match native_event {
                NativeViewEvent::ButtonTapped { id } => {
                    if Some(id) == self.native_view_id {
                        cx.widget_action(uid, &scope.path, NativeViewAction::ButtonClicked);
                    }
                }
                NativeViewEvent::TextChanged { id, text } => {
                    if Some(id) == self.native_view_id {
                        self.text = text.clone();
                        cx.widget_action(uid, &scope.path, NativeViewAction::TextChanged(text));
                    }
                }
                NativeViewEvent::SwitchChanged { id, on } => {
                    if Some(id) == self.native_view_id {
                        self.switch_on = on;
                        cx.widget_action(uid, &scope.path, NativeViewAction::SwitchChanged(on));
                    }
                }
                NativeViewEvent::SliderChanged { id, value } => {
                    if Some(id) == self.native_view_id {
                        self.slider_value = value;
                        cx.widget_action(uid, &scope.path, NativeViewAction::SliderChanged(value));
                    }
                }
                NativeViewEvent::TextureUpdated { id } => {
                    if Some(id) == self.native_view_id {
                        self.draw_bg.redraw(cx);
                    }
                }
                _ => {}
            }
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Get the exact rect from Makepad's layout system
        let rect = cx.walk_turtle(walk);
        self.last_rect = Some(rect);
        
        // Create native view if needed (with the correct frame)
        if !self.is_created {
            self.create_native_view(cx.cx);
        }
        
        // Always update native view frame to match exact layout rect
        // This ensures perfect alignment and sizing
        if let Some(id) = self.native_view_id {
            cx.cx.set_native_view_frame(id, rect);
        }
        
        // Draw the background (which will sample from the native texture)
        self.draw_bg.draw_abs(cx, rect);
        
        DrawStep::done()
    }
}

impl NativeView {
    /// Create the native view
    fn create_native_view(&mut self, cx: &mut Cx) {
        // Generate a unique ID based on widget uid
        let id = NativeViewId(LiveId(self.widget_uid().0));
        self.native_view_id = Some(id);
        
        // Build the configuration
        let config = self.build_config();
        
        // Create the native view
        if cx.create_native_view(id, config) {
            self.is_created = true;
        }
    }
    
    /// Update the native view
    fn update_native_view(&mut self, cx: &mut Cx) {
        if let Some(id) = self.native_view_id {
            let config = self.build_config();
            cx.update_native_view(id, config);
        }
    }
    
    /// Build the native view configuration
    fn build_config(&self) -> NativeViewConfig {
        let view_type = match &self.view_kind {
            NativeViewKind::Button => NativeViewType::Button { 
                label: self.label.clone() 
            },
            NativeViewKind::TextField => NativeViewType::TextField { 
                placeholder: self.placeholder.clone(),
                text: self.text.clone(),
            },
            NativeViewKind::Label => NativeViewType::Label { 
                text: self.text.clone() 
            },
            NativeViewKind::Switch => NativeViewType::Switch { 
                on: self.switch_on 
            },
            NativeViewKind::Slider => NativeViewType::Slider { 
                value: self.slider_value,
                min: self.slider_min,
                max: self.slider_max,
            },
            NativeViewKind::ProgressIndicator => NativeViewType::ProgressIndicator { 
                progress: self.progress 
            },
            NativeViewKind::Custom(type_name) => NativeViewType::Custom { 
                type_name: type_name.clone(),
                properties: std::collections::HashMap::new(),
            },
        };
        
        // Use the last known rect, or a default if not yet laid out
        // The frame will be updated immediately after creation in draw_walk
        let frame = self.last_rect.unwrap_or(Rect {
            pos: dvec2(0.0, 0.0),
            size: dvec2(200.0, 100.0), // Default size
        });
        
        NativeViewConfig {
            view_type,
            frame,
            background_color: self.background_color,
            interactive: self.interactive,
            texture_scale: self.texture_scale,
        }
    }
    
    /// Set the button label
    pub fn set_label(&mut self, cx: &mut Cx, label: &str) {
        self.label = label.to_string();
        self.update_native_view(cx);
    }
    
    /// Set the text value
    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.text = text.to_string();
        self.update_native_view(cx);
    }
    
    /// Set the switch state
    pub fn set_switch(&mut self, cx: &mut Cx, on: bool) {
        self.switch_on = on;
        self.update_native_view(cx);
    }
    
    /// Set the slider value
    pub fn set_slider_value(&mut self, cx: &mut Cx, value: f64) {
        self.slider_value = value;
        self.update_native_view(cx);
    }
    
    /// Set the progress value
    pub fn set_progress(&mut self, cx: &mut Cx, progress: f64) {
        self.progress = progress;
        self.update_native_view(cx);
    }
}

impl Drop for NativeView {
    fn drop(&mut self) {
        // Note: We can't call cx.destroy_native_view here because we don't have access to Cx
        // The native view will be cleaned up when the platform layer detects it's no longer referenced
    }
}

/// Actions emitted by NativeView
#[derive(Clone, Debug, DefaultNone)]
pub enum NativeViewAction {
    None,
    ButtonClicked,
    TextChanged(String),
    SwitchChanged(bool),
    SliderChanged(f64),
}

impl NativeViewRef {
    /// Set the button label
    pub fn set_label(&self, cx: &mut Cx, label: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_label(cx, label);
        }
    }
    
    /// Set the text value
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }
    
    /// Set the switch state
    pub fn set_switch(&self, cx: &mut Cx, on: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_switch(cx, on);
        }
    }
    
    /// Set the slider value
    pub fn set_slider_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_slider_value(cx, value);
        }
    }
    
    /// Set the progress value
    pub fn set_progress(&self, cx: &mut Cx, progress: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_progress(cx, progress);
        }
    }
    
    /// Check if a button was clicked
    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeViewAction::ButtonClicked = item.cast() {
                return true;
            }
        }
        false
    }
    
    /// Get text change if any
    pub fn text_changed(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeViewAction::TextChanged(text) = item.cast() {
                return Some(text);
            }
        }
        None
    }
    
    /// Get switch change if any
    pub fn switch_changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeViewAction::SwitchChanged(on) = item.cast() {
                return Some(on);
            }
        }
        None
    }
    
    /// Get slider change if any
    pub fn slider_changed(&self, actions: &Actions) -> Option<f64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeViewAction::SliderChanged(value) = item.cast() {
                return Some(value);
            }
        }
        None
    }
}
