use core_graphics::{
    event::{ CGEvent, CGEventType, CGEventTapLocation, CGMouseButton },
    event_source::{ CGEventSource, CGEventSourceStateID },
    geometry::CGPoint
};

use crate::inputs::{ Action, MouseButton };
pub trait Keylike: Copy {
    fn produce_input(self, action: Action) -> CGEvent;
}

impl Keylike for MouseButton {
    fn produce_input(self, action: Action) -> CGEvent {
        let event_type = match (self, action) {
            (MouseButton::Left, Action::Press) => CGEventType::LeftMouseDown,
            (MouseButton::Left, Action::Release) => CGEventType::LeftMouseUp,
            (MouseButton::Right, Action::Press) => CGEventType::RightMouseDown,
            (MouseButton::Right, Action::Release) => CGEventType::RightMouseUp,
            (MouseButton::Middle, Action::Press) => CGEventType::OtherMouseDown,
            (MouseButton::Middle, Action::Release) => CGEventType::OtherMouseUp
        };

        let button = match self {
            MouseButton::Left => CGMouseButton::Left,
            MouseButton::Right => CGMouseButton::Right,
            MouseButton::Middle => CGMouseButton::Center
        };

        let point = get_mouse_position().expect("Failed to get mouse position.");
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).expect("Failed to create event source.");
        let event = CGEvent::new_mouse_event(source, event_type, point, button).expect("Failed to create mouse event.");

        event
    }
}

fn get_mouse_position() -> Option<CGPoint> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
    let event = CGEvent::new(source).ok()?;
    let point = event.location();

    Some(CGPoint::new(point.x, point.y))
}

#[inline(always)]
pub fn send<K: Keylike>(key: K) {
    let press_event = key.produce_input(Action::Press);
    let release_event = key.produce_input(Action::Release);
    press_event.post(CGEventTapLocation::HID);
    release_event.post(CGEventTapLocation::HID);
}