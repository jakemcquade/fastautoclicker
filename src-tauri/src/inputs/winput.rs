use winapi::um::winuser;
use std::marker::Copy;

use crate::inputs::{ Action, Button };
// impl Action {
//     pub fn from_bool(is_press: bool) -> Self {
//         if is_press { Self::Press }
//         else { Self::Release }
//     }
// }

#[derive(Clone)]
#[repr(transparent)]
pub struct Input(winuser::INPUT);
impl Input {
    pub fn from_button(button: Button) -> Input {
        let mut input: winuser::INPUT = unsafe { std::mem::zeroed() };
        input.type_ = winuser::INPUT_MOUSE;
        let mi = unsafe { input.u.mi_mut() };
        mi.dwFlags = match button {
            Button::Left => winuser::MOUSEEVENTF_LEFTDOWN | winuser::MOUSEEVENTF_LEFTUP,
            Button::Right => winuser::MOUSEEVENTF_RIGHTDOWN | winuser::MOUSEEVENTF_RIGHTUP,
            Button::Middle => winuser::MOUSEEVENTF_MIDDLEDOWN | winuser::MOUSEEVENTF_MIDDLEUP,
        };
        Input(input)
    }
}

pub trait Keylike: Copy {
    fn produce_input(self, action: Action) -> Input;
}

impl Keylike for Button {
    fn produce_input(self, _action: Action) -> Input {
        Input::from_button(self)
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