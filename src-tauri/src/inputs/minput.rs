use core_graphics::{
    event::{ CGEvent, CGEventType, CGEventTapLocation, CGMouseButton },
    event_source::{ CGEventSource, CGEventSourceStateID },
    geometry::{ CGPoint }
};

use crate::inputs::{Action, Button};
pub trait Keylike: Copy {
    fn produce_input(self, action: Action) -> Result<CGEvent, std::io::Error>;
}

impl Keylike for Button {
    fn produce_input(self, action: Action) -> Result<CGEvent, std::io::Error> {
        if (action == Action::Press) {
            let event_type = match (self, action) {
                (Button::Left, Action::Press) => CGEventType::LeftMouseDown,
                (Button::Right, Action::Press) => CGEventType::RightMouseDown,
                (Button::Middle, Action::Press) => CGEventType::OtherMouseDown
            };
    
            let button = match self {
                Button::Left => CGMouseButton::Left,
                Button::Right => CGMouseButton::Right,
                Button::Middle => CGMouseButton::Center
            };
    
            let point = get_current_mouse_location().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get mouse location"))?;
            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
                .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create CGEventSource"))?;
            
            CGEvent::new_mouse_event(source, event_type, point, button)
                .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create mouse event"))
        } else {
            let event_type = match (self, action) {
                (Button::Left, Action::Release) => CGEventType::LeftMouseUp,
                (Button::Right, Action::Release) => CGEventType::RightMouseUp,
                (Button::Middle, Action::Release) => CGEventType::OtherMouseUp
            };
    
            let button = match self {
                Button::Left => CGMouseButton::Left,
                Button::Right => CGMouseButton::Right,
                Button::Middle => CGMouseButton::Center
            };
    
            let position  = match get_mouse_position() {
                Mouse::Position { x, y } => Position { x, y },
                Mouse::Error => std::io::Error::new(std::io::ErrorKind::Other, "Faield to get mouse position")
            };

            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create CGEventSource"))?;
            CGEvent::new_mouse_event(source, event_type, position, button).ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create mouse event"))
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
pub fn send<K: Keylike>(key: K) -> Result<(), io::Error> {
    let press_event = key.produce_input(Action::Press)?;
    let release_event = key.produce_input(Action::Release)?;
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to create CGEventSource"))?;
    
    press_event.post(CGEventTapLocation::HID);
    release_event.post(CGEventTapLocation::HID);
    
    Ok(())
}