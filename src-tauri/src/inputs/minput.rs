use core_graphics::event::{ CGEvent, CGEventType, CGMouseButton };
use core_graphics::event_source::{ CGEventSource, CGEventSourceStateID };
use core_graphics::geometry::CGPoint;
// use std::time::Duration;
// use std::thread::sleep;

use crate::inputs::{ Action, Button };

pub trait Keylike: Copy {
    fn produce_input(self, action: Action) -> CGEvent;
}

impl Keylike for Button {
    fn produce_input(self, action: Action) -> CGEvent {
        let event_type = match (self, action) {
            (Button::Left, Action::Press) => CGEventType::LeftMouseDown,
            (Button::Left, Action::Release) => CGEventType::LeftMouseUp,
            (Button::Right, Action::Press) => CGEventType::RightMouseDown,
            (Button::Right, Action::Release) => CGEventType::RightMouseUp,
            (Button::Middle, Action::Press) => CGEventType::OtherMouseDown,
            (Button::Middle, Action::Release) => CGEventType::OtherMouseUp,
        };

        let button = match self {
            Button::Left => CGMouseButton::Left,
            Button::Right => CGMouseButton::Right,
            Button::Middle => CGMouseButton::Center,
        };

        let point = get_current_mouse_location().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get mouse location"))?;
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState);
        CGEvent::new_mouse_event(source, event_type, point, button).unwrap()
    }
}

unsafe fn get_current_mouse_location() -> Option<CGPoint> {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok()?;
    let event = CGEvent::new(source).ok()?;
    Some(event.location())
}

#[inline(always)]
pub fn send<K: Keylike>(key: K) {
    let press_event = key.produce_input(Action::Press);
    let release_event = key.produce_input(Action::Release);

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState);
    press_event.post(source)?;
    // sleep(Duration::from_millis(1));
    release_event.post(source)?;
}