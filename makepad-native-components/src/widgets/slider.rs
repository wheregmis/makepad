use crate::core::native_view::{NativeViewHost, NativeWidget};
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    link native_components;
    use makepad_draw::shader::std::*;

    DrawNativeSlider = {{DrawNativeSlider}} {
        texture native_texture: texture2d

        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }

    pub NativeSlider = {{NativeSlider}} {
        width: 300,
        height: 32,
        draw_bg: { class: DrawNativeSlider },
        slider_value: 0.5,
        slider_min: 0.0,
        slider_max: 1.0
    }
}

#[derive(Live, LiveHook, LiveRegister, Default)]
pub struct SliderInner {
    #[live]
    pub slider_value: f64,
    #[live]
    pub slider_min: f64,
    #[live]
    pub slider_max: f64,
}

impl NativeWidget for SliderInner {
    type Action = NativeSliderAction;

    fn view_type(&self) -> &'static str {
        "slider"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("value".to_string(), self.slider_value.to_string());
        props.insert("min".to_string(), self.slider_min.to_string());
        props.insert("max".to_string(), self.slider_max.to_string());
        props
    }

    fn apply_event(
        &mut self,
        event: &makepad_platform::native_view::NativeViewEvent,
    ) -> Option<Self::Action> {
        if event.kind.as_str() == "slider_changed" {
            if let Some(value) = event.data.get("value").and_then(|v| v.parse::<f64>().ok()) {
                self.slider_value = value;
                return Some(NativeSliderAction::ValueChanged(value));
            }
        }
        None
    }
}

#[derive(Live, Widget)]
pub struct NativeSlider {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    pub draw_bg: DrawNativeSlider,

    #[live]
    pub slider_value: f64,
    #[live]
    pub slider_min: f64,
    #[live]
    pub slider_max: f64,

    #[rust]
    host: NativeViewHost<SliderInner>,
}

impl LiveHook for NativeSlider {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.host.inner.slider_value = self.slider_value;
        self.host.inner.slider_min = self.slider_min;
        self.host.inner.slider_max = self.slider_max;
        if self.host.is_created {
            self.host.update_view(cx);
        }
    }
}

impl Widget for NativeSlider {
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

impl NativeSlider {
    pub fn set_value(&mut self, cx: &mut Cx, value: f64) {
        self.host.inner.slider_value = value;
        self.slider_value = value;
        self.host.update_view(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NativeSliderAction {
    None,
    ValueChanged(f64),
}

impl NativeSliderRef {
    pub fn set_value(&self, cx: &mut Cx, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_value(cx, value);
        }
    }

    pub fn value_changed(&self, actions: &Actions) -> Option<f64> {
        if let Some(item) = actions.find_widget_action(self.widget_uid()) {
            if let NativeSliderAction::ValueChanged(v) = item.cast() {
                return Some(v);
            }
        }
        None
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeSlider {
    #[deref]
    pub draw_super: DrawQuad,
}
