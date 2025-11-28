use crate::core::native_view::{NativeViewHost, NativeWidget};
use makepad_widgets::*;
use std::collections::HashMap;

live_design! {
    link native_components;
    use makepad_draw::shader::std::*;

    DrawNativeProgress = {{DrawNativeProgress}} {
        texture native_texture: texture2d

        fn pixel(self) -> vec4 {
            let color = sample2d(self.native_texture, self.pos);
            return Pal::premul(color);
        }
    }

    pub NativeProgress = {{NativeProgress}} {
        width: 300,
        height: 8,
        draw_bg: { class: DrawNativeProgress },
        progress: 0.3
    }
}

#[derive(Live, LiveHook, LiveRegister, Default)]
pub struct ProgressInner {
    #[live]
    pub progress: f64,
}

impl NativeWidget for ProgressInner {
    type Action = NativeProgressAction;

    fn view_type(&self) -> &'static str {
        "progress_indicator"
    }

    fn properties(&self) -> HashMap<String, String> {
        let mut props = HashMap::new();
        props.insert("progress".to_string(), self.progress.to_string());
        props
    }

    fn apply_event(
        &mut self,
        _event: &makepad_platform::native_view::NativeViewEvent,
    ) -> Option<Self::Action> {
        None
    }
}

#[derive(Live, Widget)]
pub struct NativeProgress {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    pub draw_bg: DrawNativeProgress,

    #[live]
    pub progress: f64,

    #[rust]
    host: NativeViewHost<ProgressInner>,
}

impl LiveHook for NativeProgress {
    fn after_apply(&mut self, cx: &mut Cx, _apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        self.host.inner.progress = self.progress;
        if self.host.is_created {
            self.host.update_view(cx);
        }
    }
}

impl Widget for NativeProgress {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();

        if self.host.inner.interactive() && self.host.next_frame_matches(event) {
            self.host
                .process_events(cx, scope, uid, &mut self.draw_bg.draw_super);
            self.host.schedule_next_frame(cx);
        }

        // Progress is non-interactive; still process events for texture updates.
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

        DrawStep::done()
    }
}

impl NativeProgress {
    pub fn set_progress(&mut self, cx: &mut Cx, progress: f64) {
        self.host.inner.progress = progress;
        self.progress = progress;
        self.host.update_view(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum NativeProgressAction {
    None,
}

impl NativeProgressRef {
    pub fn set_progress(&self, cx: &mut Cx, progress: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_progress(cx, progress);
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawNativeProgress {
    #[deref]
    pub draw_super: DrawQuad,
}
