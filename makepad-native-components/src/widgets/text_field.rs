use crate::core::native_view::{NativeViewHost, NativeWidget};
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    link native_components;
    use makepad_draw::shader::std::*;

    DrawNativeTextField = {{DrawNativeTextField}} {
        texture native_texture: texture2d

        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }

    pub NativeTextField = {{NativeTextField}} {
        width: 280,
        height: 36,
        draw_bg: { class: DrawNativeTextField },
        placeholder: "Enter text...",
        text: ""
    }
}

#[derive(Live, LiveHook, LiveRegister, Default)]
pub struct TextFieldInner {
    #[live]
    pub placeholder: String,
    #[live]
    pub text: String,
}

impl NativeWidget for TextFieldInner {
    type Action = NativeTextFieldAction;

    fn view_type(&self) -> &'static str {
        "text_field"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("placeholder".to_string(), self.placeholder.clone());
        props.insert("text".to_string(), self.text.clone());
        props
    }

    fn apply_event(
        &mut self,
        event: &makepad_platform::native_view::NativeViewEvent,
    ) -> Option<Self::Action> {
        if event.kind.as_str() == "text_changed" {
            if let Some(text) = event.data.get("text").cloned() {
                self.text = text.clone();
                return Some(NativeTextFieldAction::TextChanged(text));
            }
        }
        None
    }
}

#[derive(Live, Widget)]
pub struct NativeTextField {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    pub draw_bg: DrawNativeTextField,

    #[live]
    pub placeholder: String,
    #[live]
    pub text: String,

    #[rust]
    host: NativeViewHost<TextFieldInner>,
}

impl LiveHook for NativeTextField {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.host.inner.placeholder = self.placeholder.clone();
        self.host.inner.text = self.text.clone();
        if self.host.is_created {
            self.host.update_view(cx);
        }
    }
}

impl Widget for NativeTextField {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.host.inner.interactive() && self.host.next_frame_matches(event) {
            self.host
                .process_events(cx, scope, uid, &mut self.draw_bg.draw_super);
            self.host.schedule_next_frame(cx);
        }

        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) => self
                .host
                .handle_touch(cx, fe.abs, NativeViewTouchPhase::Began),
            Hit::FingerMove(fe) => self
                .host
                .handle_touch(cx, fe.abs, NativeViewTouchPhase::Moved),
            Hit::FingerUp(fe) => self
                .host
                .handle_touch(cx, fe.abs, NativeViewTouchPhase::Ended),
            _ => {}
        }

        self.host
            .process_events(cx, scope, uid, &mut self.draw_bg.draw_super);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle(walk);
        self.host.last_rect = Some(rect);

        self.host.ensure_created(cx.cx, self.widget_uid());
        cx.add_aligned_rect_area(&mut self.draw_bg.draw_super.draw_vars.area, rect);
        self.host
            .set_frame_from_area(cx.cx, &self.draw_bg.draw_super.draw_vars.area);

        #[cfg(not(target_os = "macos"))]
        {
            self.draw_bg.draw_abs(cx, rect);
        }

        if self.host.inner.interactive() {
            self.host.schedule_next_frame(cx.cx);
        }

        DrawStep::done()
    }
}

impl NativeTextField {
    pub fn set_text(&mut self, cx: &mut Cx, text: &str) {
        self.host.inner.text = text.to_string();
        self.text = text.to_string();
        self.host.update_view(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NativeTextFieldAction {
    None,
    TextChanged(String),
}

impl NativeTextFieldRef {
    pub fn set_text(&self, cx: &mut Cx, text: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_text(cx, text);
        }
    }

    pub fn text_changed(&self, actions: &Actions) -> Option<String> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeTextFieldAction::TextChanged(text) = item.cast() {
                return Some(text);
            }
        }
        None
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeTextField {
    #[deref]
    pub draw_super: DrawQuad,
}
