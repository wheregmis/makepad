use crate::core::native_view::{NativeViewHost, NativeWidget};
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    link native_components;
    use makepad_draw::shader::std::*;

    DrawNativeButton = {{DrawNativeButton}} {
        texture native_texture: texture2d

        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }

    pub NativeButton = {{NativeButton}} {
        width: 140,
        height: 36,
        draw_bg: { class: DrawNativeButton },
        label: "Click Me!"
    }
}

#[derive(Live, LiveHook, LiveRegister, Default)]
pub struct ButtonInner {
    #[live]
    pub label: String,
}

impl NativeWidget for ButtonInner {
    type Action = NativeButtonAction;

    fn view_type(&self) -> &'static str {
        "button"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("label".to_string(), self.label.clone());
        props
    }

    fn apply_event(
        &mut self,
        event: &makepad_platform::native_view::NativeViewEvent,
    ) -> Option<Self::Action> {
        if event.kind.as_str() == "button_tapped" {
            return Some(NativeButtonAction::Clicked);
        }
        None
    }
}

#[derive(Live, Widget)]
pub struct NativeButton {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    pub draw_bg: DrawNativeButton,

    #[live]
    pub label: String,

    #[rust]
    host: NativeViewHost<ButtonInner>,
}

impl LiveHook for NativeButton {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.host.inner.label = self.label.clone();
        if self.host.is_created {
            self.host.update_view(cx);
        }
    }
}

impl Widget for NativeButton {
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

impl NativeButton {
    pub fn set_label(&mut self, cx: &mut Cx, label: &str) {
        self.host.inner.label = label.to_string();
        self.label = label.to_string();
        self.host.update_view(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NativeButtonAction {
    None,
    Clicked,
}

impl NativeButtonRef {
    pub fn set_label(&self, cx: &mut Cx, label: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_label(cx, label);
        }
    }

    pub fn clicked(&self, actions: &Actions) -> bool {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeButtonAction::Clicked = item.cast() {
                return true;
            }
        }
        false
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeButton {
    #[deref]
    pub draw_super: DrawQuad,
}
