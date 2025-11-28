use makepad_platform::native_view::*;
use makepad_widgets::*;
use std::collections::HashMap;

/// Trait implemented by each native widget wrapper.
pub trait NativeWidget {
    type Action: WidgetActionTrait + Clone + Default + 'static;

    fn view_type(&self) -> &'static str;
    fn properties(&self) -> HashMap<String, String>;
    fn apply_event(&mut self, event: &NativeViewEvent) -> Option<Self::Action>;
    fn interactive(&self) -> bool {
        true
    }
    fn background_color(&self) -> Option<Vec4> {
        None
    }
    fn texture_scale(&self) -> f64 {
        2.0
    }
}

/// Shared host that manages platform native view lifecycle and event polling.
#[derive(Default)]
pub struct NativeViewHost<W: NativeWidget + Default> {
    pub inner: W,
    pub native_view_id: Option<NativeViewId>,
    pub is_created: bool,
    pub last_rect: Option<Rect>,
    pub next_frame: NextFrame,
}

impl<W: NativeWidget + Default> NativeViewHost<W> {
    pub fn ensure_created(&mut self, cx: &mut Cx, widget_uid: WidgetUid) {
        if self.is_created {
            return;
        }
        let id = NativeViewId(LiveId(widget_uid.0));
        self.native_view_id = Some(id);
        let config = self.build_config();
        if cx.create_native_view(id, config) {
            self.is_created = true;
        }
    }

    pub fn update_view(&mut self, cx: &mut Cx) {
        if let Some(id) = self.native_view_id {
            let config = self.build_config();
            cx.update_native_view(id, config);
        }
    }

    pub fn set_frame(&mut self, cx: &mut Cx, rect: Rect, clip: Rect) {
        self.last_rect = Some(rect);
        if let Some(id) = self.native_view_id {
            cx.set_native_view_frame(id, rect, clip);
        }
    }

    /// Set the native view frame from an Area, respecting clipping/scrolling.
    pub fn set_frame_from_area(&mut self, cx: &mut Cx, area: &Area) {
        // Use both unclipped and clipped rects to derive a visible frame.
        let unclipped = area.rect(cx);
        let mut clipped = area.clipped_rect(cx);

        // Gather draw-list transforms.
        let mut shift = dvec2(0.0, 0.0);
        let mut view_clip: Option<Rect> = None;
        if let Some(draw_list_id) = area.draw_list_id() {
            if let Some(draw_list) = cx.draw_lists.checked_index(draw_list_id) {
                shift = dvec2(
                    draw_list.draw_list_uniforms.view_shift.x as f64,
                    draw_list.draw_list_uniforms.view_shift.y as f64,
                );
                if draw_list.draw_list_has_clip {
                    let vc = draw_list.draw_list_uniforms.view_clip;
                    view_clip = Some(Rect {
                        pos: dvec2(vc.x as f64, vc.y as f64),
                        size: dvec2((vc.z - vc.x) as f64, (vc.w - vc.y) as f64),
                    });
                }
            }
        }

        // Transform rects into screen space using the shift.
        let unclipped_screen = Rect {
            pos: unclipped.pos + shift,
            size: unclipped.size,
        };
        let mut clipped_screen = Rect {
            pos: clipped.pos + shift,
            size: clipped.size,
        };

        // If clipped rect is empty, fall back to view-clip intersection (if any), otherwise use the unclipped rect.
        if (clipped_screen.size.x <= 0.0 || clipped_screen.size.y <= 0.0)
            && (unclipped_screen.size.x > 0.0 && unclipped_screen.size.y > 0.0)
        {
            if let Some(vc) = view_clip {
                clipped_screen = unclipped_screen.clip((vc.pos, vc.pos + vc.size));
            } else {
                clipped_screen = unclipped_screen;
            }
        }

        crate::log!(
            "NativeView layout: unclipped={:?} clipped={:?} unclipped_screen={:?} clipped_screen={:?}",
            unclipped,
            clipped,
            unclipped_screen,
            clipped_screen
        );

        self.set_frame(cx, unclipped_screen, clipped_screen);
    }

    pub fn schedule_next_frame(&mut self, cx: &mut Cx) {
        self.next_frame = cx.new_next_frame();
    }

    pub fn next_frame_matches(&self, event: &Event) -> bool {
        self.next_frame.is_event(event).is_some()
    }

    pub fn process_events(&mut self, cx: &mut Cx, scope: &mut Scope, uid: WidgetUid, draw_bg: &mut DrawQuad) {
        if let Some(id) = self.native_view_id {
            let events = cx.drain_native_view_events_for(id);
            for native_event in events {
                if let Some(action) = self.inner.apply_event(&native_event) {
                    cx.widget_action(uid, &scope.path, action);
                }
                if native_event.kind == "texture_updated" {
                    draw_bg.redraw(cx);
                }
            }
        }
    }

    pub fn handle_touch(&self, cx: &mut Cx, abs: DVec2, phase: NativeViewTouchPhase) {
        if let Some((id, rect)) = self.native_view_id.zip(self.last_rect) {
            let local_pos = abs - rect.pos;
            cx.send_touch_to_native_view(NativeViewTouchEvent {
                id,
                phase,
                position: local_pos,
                time: 0.0,
            });
        }
    }

    fn build_config(&self) -> NativeViewConfig {
        let mut frame = self.last_rect.unwrap_or(Rect {
            pos: dvec2(0.0, 0.0),
            size: dvec2(60.0, 32.0),
        });

        if !frame.size.x.is_finite() || !frame.size.y.is_finite() || frame.size.x <= 0.0 || frame.size.y <= 0.0 {
            frame.size = dvec2(60.0, 32.0);
        }

        NativeViewConfig {
            view_type: self.inner.view_type().to_string(),
            properties: self.inner.properties(),
            frame,
            background_color: self.inner.background_color(),
            interactive: self.inner.interactive(),
            texture_scale: self.inner.texture_scale(),
        }
    }
}
