use crate::makepad_draw::{
    event::{finger::LongPressEvent, KeyModifiers, TouchPoint, TouchState, TouchUpdateEvent},
    *,
};

pub const TOUCH_ACTIVATION_SLOP: f64 = 12.0;
pub const TOUCH_ACTIVATION_TIME: f64 = 0.5;

#[derive(Clone, Debug)]
pub struct TouchActivationInfo {
    pub window_id: WindowId,
    pub abs: Vec2d,
    pub modifiers: KeyModifiers,
    pub time: f64,
    pub uid: u64,
}

#[derive(Clone, Debug)]
pub struct TouchActivationRelease {
    pub window_id: WindowId,
    pub abs: Vec2d,
    pub abs_start: Vec2d,
    pub modifiers: KeyModifiers,
    pub time: f64,
    pub uid: u64,
    pub is_over: bool,
    pub was_tap: bool,
    pub was_long_press: bool,
}

#[derive(Clone, Debug)]
pub enum TouchActivationEvent {
    None,
    Started(TouchActivationInfo),
    LongPress(TouchActivationInfo),
    Released(TouchActivationRelease),
    Canceled(TouchActivationInfo),
}

impl Default for TouchActivationEvent {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug)]
struct TouchActivationCandidate {
    window_id: WindowId,
    abs_start: Vec2d,
    modifiers: KeyModifiers,
    time: f64,
    uid: u64,
    long_press_occurred: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TouchActivation {
    candidate: Option<TouchActivationCandidate>,
}

impl TouchActivation {
    pub fn is_active(&self) -> bool {
        self.candidate.is_some()
    }

    pub fn cancel(&mut self) {
        self.candidate = None;
    }

    pub fn handle_event<F>(
        &mut self,
        event: &Event,
        handle_area: Area,
        hit_test: F,
    ) -> TouchActivationEvent
    where
        F: Fn(Vec2d) -> bool,
    {
        match event {
            Event::TouchUpdate(update) => self.handle_touch_update(update, handle_area, hit_test),
            Event::LongPress(long_press) => self.handle_long_press(long_press),
            _ => TouchActivationEvent::None,
        }
    }

    fn handle_touch_update<F>(
        &mut self,
        update: &TouchUpdateEvent,
        handle_area: Area,
        hit_test: F,
    ) -> TouchActivationEvent
    where
        F: Fn(Vec2d) -> bool,
    {
        if let Some(candidate) = &self.candidate {
            if let Some(touch) = update
                .touches
                .iter()
                .find(|touch| touch.uid == candidate.uid)
            {
                return self.handle_candidate_touch(update, touch, hit_test);
            }
        }

        if self.candidate.is_none() {
            for touch in &update.touches {
                if touch.state != TouchState::Start || !touch.handled.get().is_empty() {
                    continue;
                }
                if !hit_test(touch.abs) {
                    continue;
                }
                touch.handled.set(handle_area);
                self.candidate = Some(TouchActivationCandidate {
                    window_id: update.window_id,
                    abs_start: touch.abs,
                    modifiers: update.modifiers,
                    time: touch.time,
                    uid: touch.uid,
                    long_press_occurred: false,
                });
                return TouchActivationEvent::Started(TouchActivationInfo {
                    window_id: update.window_id,
                    abs: touch.abs,
                    modifiers: update.modifiers,
                    time: touch.time,
                    uid: touch.uid,
                });
            }
        }

        TouchActivationEvent::None
    }

    fn handle_candidate_touch<F>(
        &mut self,
        update: &TouchUpdateEvent,
        touch: &TouchPoint,
        hit_test: F,
    ) -> TouchActivationEvent
    where
        F: Fn(Vec2d) -> bool,
    {
        let Some(candidate) = self.candidate.clone() else {
            return TouchActivationEvent::None;
        };

        match touch.state {
            TouchState::Move => {
                if touch.abs.distance(&candidate.abs_start) > TOUCH_ACTIVATION_SLOP {
                    self.candidate = None;
                    TouchActivationEvent::Canceled(TouchActivationInfo {
                        window_id: candidate.window_id,
                        abs: touch.abs,
                        modifiers: candidate.modifiers,
                        time: touch.time,
                        uid: touch.uid,
                    })
                } else {
                    TouchActivationEvent::None
                }
            }
            TouchState::Stop => {
                self.candidate = None;
                let was_tap = !candidate.long_press_occurred
                    && (touch.time - candidate.time) < TOUCH_ACTIVATION_TIME
                    && touch.abs.distance(&candidate.abs_start) < TOUCH_ACTIVATION_SLOP;
                TouchActivationEvent::Released(TouchActivationRelease {
                    window_id: update.window_id,
                    abs: touch.abs,
                    abs_start: candidate.abs_start,
                    modifiers: update.modifiers,
                    time: touch.time,
                    uid: touch.uid,
                    is_over: hit_test(touch.abs),
                    was_tap,
                    was_long_press: candidate.long_press_occurred,
                })
            }
            TouchState::Start | TouchState::Stable => TouchActivationEvent::None,
        }
    }

    fn handle_long_press(&mut self, long_press: &LongPressEvent) -> TouchActivationEvent {
        let Some(candidate) = self.candidate.as_mut() else {
            return TouchActivationEvent::None;
        };
        if candidate.uid != long_press.uid {
            return TouchActivationEvent::None;
        }
        candidate.long_press_occurred = true;
        TouchActivationEvent::LongPress(TouchActivationInfo {
            window_id: long_press.window_id,
            abs: long_press.abs,
            modifiers: KeyModifiers::default(),
            time: long_press.time,
            uid: long_press.uid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn make_touch(uid: u64, state: TouchState, x: f64, y: f64, time: f64) -> TouchPoint {
        TouchPoint {
            state,
            abs: dvec2(x, y),
            time,
            uid,
            rotation_angle: 0.0,
            force: 0.0,
            radius: dvec2(0.0, 0.0),
            handled: Cell::new(Area::Empty),
            sweep_lock: Cell::new(Area::Empty),
        }
    }

    fn touch_event(touches: Vec<TouchPoint>, time: f64) -> Event {
        Event::TouchUpdate(TouchUpdateEvent {
            time,
            window_id: WindowId(0, 0),
            modifiers: KeyModifiers::default(),
            touches,
        })
    }

    #[test]
    fn confirms_valid_tap() {
        let mut activation = TouchActivation::default();
        let area = Area::Empty;

        let started = activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Start, 10.0, 10.0, 0.0)], 0.0),
            area,
            |_| true,
        );
        assert!(matches!(started, TouchActivationEvent::Started(_)));

        let released = activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Stop, 14.0, 12.0, 0.2)], 0.2),
            area,
            |_| true,
        );
        match released {
            TouchActivationEvent::Released(release) => {
                assert!(release.was_tap);
                assert!(release.is_over);
            }
            _ => panic!("expected release"),
        }
    }

    #[test]
    fn cancels_after_moving_past_slop() {
        let mut activation = TouchActivation::default();
        let area = Area::Empty;

        activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Start, 10.0, 10.0, 0.0)], 0.0),
            area,
            |_| true,
        );

        let canceled = activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Move, 30.0, 10.0, 0.1)], 0.1),
            area,
            |_| true,
        );
        assert!(matches!(canceled, TouchActivationEvent::Canceled(_)));
        assert!(!activation.is_active());
    }

    #[test]
    fn release_outside_target_is_not_valid_activation() {
        let mut activation = TouchActivation::default();
        let area = Area::Empty;

        activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Start, 10.0, 10.0, 0.0)], 0.0),
            area,
            |_| true,
        );

        let released = activation.handle_event(
            &touch_event(vec![make_touch(1, TouchState::Stop, 10.0, 10.0, 0.2)], 0.2),
            area,
            |_| false,
        );
        match released {
            TouchActivationEvent::Released(release) => {
                assert!(release.was_tap);
                assert!(!release.is_over);
            }
            _ => panic!("expected release"),
        }
    }

    #[test]
    fn long_press_prevents_tap_release() {
        let mut activation = TouchActivation::default();
        let area = Area::Empty;

        activation.handle_event(
            &touch_event(vec![make_touch(7, TouchState::Start, 10.0, 10.0, 0.0)], 0.0),
            area,
            |_| true,
        );

        let long_press = activation.handle_event(
            &Event::LongPress(LongPressEvent {
                window_id: WindowId(0, 0),
                abs: dvec2(10.0, 10.0),
                uid: 7,
                time: 0.5,
            }),
            area,
            |_| true,
        );
        assert!(matches!(long_press, TouchActivationEvent::LongPress(_)));

        let released = activation.handle_event(
            &touch_event(vec![make_touch(7, TouchState::Stop, 10.0, 10.0, 0.6)], 0.6),
            area,
            |_| true,
        );
        match released {
            TouchActivationEvent::Released(release) => {
                assert!(!release.was_tap);
                assert!(release.was_long_press);
            }
            _ => panic!("expected release"),
        }
    }
}
