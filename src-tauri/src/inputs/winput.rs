use winapi::um::winuser;
use std::marker::Copy;

use crate::inputs::{ Action, MouseButton };

#[derive(Clone)]
#[repr(transparent)]
pub struct Input(winuser::INPUT);
impl Input {
    pub fn from_button(button: MouseButton, action: Action) -> Input {
        let mut input: winuser::INPUT = unsafe { std::mem::zeroed() };
        input.type_ = winuser::INPUT_MOUSE;

        let mi = unsafe { input.u.mi_mut() };
        mi.dwFlags = match (button, action) {
            (MouseButton::Left, Action::Press) => winuser::MOUSEEVENTF_LEFTDOWN,
            (MouseButton::Left, Action::Release) => winuser::MOUSEEVENTF_LEFTUP,
            (MouseButton::Right, Action::Press) => winuser::MOUSEEVENTF_RIGHTDOWN,
            (MouseButton::Right, Action::Release) => winuser::MOUSEEVENTF_RIGHTUP,
            (MouseButton::Middle, Action::Press) => winuser::MOUSEEVENTF_MIDDLEDOWN,
            (MouseButton::Middle, Action::Release) => winuser::MOUSEEVENTF_MIDDLEUP,
        };

        Input(input)
    }
}

pub trait Keylike: Copy {
    fn produce_input(self, action: Action) -> Input;
}

impl Keylike for MouseButton {
    fn produce_input(self, action: Action) -> Input {
        Input::from_button(self, action)
    }
}

pub fn set_position(x: i32, y: i32) {
    unsafe {
        winuser::SetCursorPos(x, y);
    }
}

#[inline(always)]
pub fn send<K: Keylike>(key: K) {
    let inputs = [key.produce_input(Action::Press), key.produce_input(Action::Release)];
    send_inputs(&inputs);
}

fn send_inputs(inputs: impl AsRef<[Input]>) -> u32 {
    unsafe {
        winuser::SendInput(
            inputs.as_ref().len() as _,
            inputs.as_ref().as_ptr() as _,
            std::mem::size_of::<winuser::INPUT>() as _
        )
    }
}
