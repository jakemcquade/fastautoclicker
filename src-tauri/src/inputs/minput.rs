use core_graphics::{
    event::{ CGEvent, CGEventType, CGEventTapLocation, CGMouseButton },
    event_source::{ CGEventSource, CGEventSourceStateID },
    geometry::CGPoint
};

use crate::inputs::{Action, Button};
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
            (Button::Middle, Action::Release) => CGEventType::OtherMouseUp
        };

        let button = match self {
            Button::Left => CGMouseButton::Left,
            Button::Right => CGMouseButton::Right,
            Button::Middle => CGMouseButton::Center
        };

        let point = get_mouse_position().expect("Failed to get mouse position.");
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).expect("Failed to create event source.");
        let event = CGEvent::new_mouse_event(source, event_type, point, button).expect("Failed to create mouse event.");

        event;
    }
}

fn get_mouse_position() -> Option<CGPoint> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
    let event = CGEvent::new(source).ok()?;
    let point = event.location();

    Some(CGPoint::new(point.x, point.y))
}

#[inline(always)]
pub fn send<K: Keylike>(key: K) -> Result<(), ()> {
    let press_event = key.produce_input(Action::Press);
    let release_event = key.produce_input(Action::Release);
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;

    press_event.post(CGEventTapLocation::HID);
    release_event.post(CGEventTapLocation::HID);
    
    Ok(())
}