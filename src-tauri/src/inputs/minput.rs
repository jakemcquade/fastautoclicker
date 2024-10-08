use core_graphics::{
    event::{ CGEvent, CGEventType, CGEventTapLocation, CGMouseButton },
    event_source::{ CGEventSource, CGEventSourceStateID }
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

        if action == Action::Press {
            let point = get_mouse_position();
            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState);
            CGEvent::new_mouse_event(source, event_type, point, button)
        } else {    
            let position  = match get_mouse_position() {
                Mouse::Position { x, y } => Position { x, y },
                Mouse::Error => Position { x: 0, y: 0 }
            };

            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState);
            CGEvent::new_mouse_event(source, event_type, position, button);
        }
    }
}

pub struct Position { pub x: i32, pub y: i32 }
pub enum Mouse { Position { x: i32, y: i32 }, Error }
fn get_mouse_position() -> Mouse {
    let event = CGEvent::new(CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap());
    let point = match event {
        Ok(event) => {
            let point = event.location();
            Mouse::Position { x: point.x as i32, y: point.y as i32 }
        },
        Err(_) => return Mouse::Error,
    };

    point
}

#[inline(always)]
pub fn send<K: Keylike>(key: K) -> Result<(), std::io::Error> {
    let press_event = key.produce_input(Action::Press)?;
    let release_event = key.produce_input(Action::Release)?;
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;

    press_event.post(CGEventTapLocation::HID);
    release_event.post(CGEventTapLocation::HID);
    
    Ok(())
}