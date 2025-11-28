use crate::core::native_view::{NativeViewHost, NativeWidget};
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    link native_components;
    use makepad_draw::shader::std::*;

    DrawNativeSwitch = {{DrawNativeSwitch}} {
        texture native_texture: texture2d

        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }

    pub NativeSwitch = {{NativeSwitch}} {
        width: 60,
        height: 32,
        draw_bg: { class: DrawNativeSwitch }
    }
}

#[derive(Live, LiveHook, LiveRegister, Default)]
pub struct SwitchInner {
    #[live]
    pub switch_on: bool,
}

impl NativeWidget for SwitchInner {
    type Action = NativeSwitchAction;

    fn view_type(&self) -> &'static str {
        "switch"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("on".to_string(), self.switch_on.to_string());
        props
    }

    fn apply_event(
        &mut self,
        event: &makepad_platform::native_view::NativeViewEvent,
    ) -> Option<Self::Action> {
        match event.kind.as_str() {
            "switch_changed" => {
                if let Some(on) = event.data.get("on").and_then(|v| v.parse::<bool>().ok()) {
                    self.switch_on = on;
                    return Some(NativeSwitchAction::SwitchChanged(on));
                }
            }
            _ => {}
        }
        None
    }
}

#[derive(Live, Widget)]
pub struct NativeSwitch {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    pub draw_bg: DrawNativeSwitch,

    /// Exposed switch state for live design
    #[live(false)]
    pub switch_on: bool,

    #[rust]
    host: NativeViewHost<SwitchInner>,
}

impl LiveHook for NativeSwitch {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.host.inner.switch_on = self.switch_on;
        if self.host.is_created {
            self.host.update_view(cx);
        }
    }
}

impl Widget for NativeSwitch {
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

impl NativeSwitch {
    pub fn set_switch(&mut self, cx: &mut Cx, on: bool) {
        self.host.inner.switch_on = on;
        self.host.update_view(cx);
    }

    pub fn refresh(&mut self, cx: &mut Cx) {
        self.host.update_view(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NativeSwitchAction {
    None,
    SwitchChanged(bool),
}

impl NativeSwitchRef {
    pub fn set_switch(&self, cx: &mut Cx, on: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_switch(cx, on);
        }
    }

    pub fn switch_changed(&self, actions: &Actions) -> Option<bool> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeSwitchAction::SwitchChanged(on) = item.cast() {
                return Some(on);
            }
        }
        None
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeSwitch {
    #[deref]
    pub draw_super: DrawQuad,
}
