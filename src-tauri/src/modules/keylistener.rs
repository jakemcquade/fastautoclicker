use std::sync::{Arc, Mutex};

const MODIFIERS: &[&str] = &["Control", "Shift", "Alt", "Meta"];

#[cfg(target_os = "linux")]
mod platform {
    use evdev::{Device, InputEventKind, Key};
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};

    fn canonical(key: Key) -> Option<&'static str> {
        Some(match key {
            Key::KEY_LEFTCTRL  | Key::KEY_RIGHTCTRL  => "Control",
            Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => "Shift",
            Key::KEY_LEFTALT   | Key::KEY_RIGHTALT   => "Alt",
            Key::KEY_LEFTMETA  | Key::KEY_RIGHTMETA  => "Meta",

            Key::KEY_A => "a", Key::KEY_B => "b", Key::KEY_C => "c", Key::KEY_D => "d",
            Key::KEY_E => "e", Key::KEY_F => "f", Key::KEY_G => "g", Key::KEY_H => "h",
            Key::KEY_I => "i", Key::KEY_J => "j", Key::KEY_K => "k", Key::KEY_L => "l",
            Key::KEY_M => "m", Key::KEY_N => "n", Key::KEY_O => "o", Key::KEY_P => "p",
            Key::KEY_Q => "q", Key::KEY_R => "r", Key::KEY_S => "s", Key::KEY_T => "t",
            Key::KEY_U => "u", Key::KEY_V => "v", Key::KEY_W => "w", Key::KEY_X => "x",
            Key::KEY_Y => "y", Key::KEY_Z => "z",

            Key::KEY_0 => "0", Key::KEY_1 => "1", Key::KEY_2 => "2", Key::KEY_3 => "3",
            Key::KEY_4 => "4", Key::KEY_5 => "5", Key::KEY_6 => "6", Key::KEY_7 => "7",
            Key::KEY_8 => "8", Key::KEY_9 => "9",

            Key::KEY_F1  => "F1",  Key::KEY_F2  => "F2",  Key::KEY_F3  => "F3",
            Key::KEY_F4  => "F4",  Key::KEY_F5  => "F5",  Key::KEY_F6  => "F6",
            Key::KEY_F7  => "F7",  Key::KEY_F8  => "F8",  Key::KEY_F9  => "F9",
            Key::KEY_F10 => "F10", Key::KEY_F11 => "F11", Key::KEY_F12 => "F12",

            Key::KEY_TAB       => "Tab",
            Key::KEY_ENTER     => "Enter",
            Key::KEY_ESC       => "Escape",
            Key::KEY_BACKSPACE => "Backspace",
            Key::KEY_CAPSLOCK  => "CapsLock",
            Key::KEY_SPACE     => "Space",
            Key::KEY_DELETE    => "Delete",
            Key::KEY_INSERT    => "Insert",
            Key::KEY_HOME      => "Home",
            Key::KEY_END       => "End",
            Key::KEY_PAGEUP    => "PageUp",
            Key::KEY_PAGEDOWN  => "PageDown",
            Key::KEY_UP        => "ArrowUp",
            Key::KEY_DOWN      => "ArrowDown",
            Key::KEY_LEFT      => "ArrowLeft",
            Key::KEY_RIGHT     => "ArrowRight",

            _ => return None,
        })
    }

    pub fn spawn(hotkey: Arc<Mutex<String>>, on_trigger: impl Fn() + Send + Sync + 'static) {
        log::info!("Starting global key listener (evdev)");

        let keyboards: Vec<Device> = evdev::enumerate()
            .filter_map(|(_, d)| {
                d.supported_keys()
                    .map(|keys| keys.contains(Key::KEY_A))
                    .unwrap_or(false)
                    .then_some(d)
            })
            .collect();

        if keyboards.is_empty() {
            log::error!("No keyboard devices found — ensure the user is in the 'input' group");
            return;
        }

        let on_trigger = Arc::new(on_trigger);
        let pressed: Arc<Mutex<HashSet<&'static str>>> = Arc::new(Mutex::new(HashSet::new()));

        for mut device in keyboards {
            let hotkey = Arc::clone(&hotkey);
            let on_trigger = Arc::clone(&on_trigger);
            let pressed = Arc::clone(&pressed);
            let name = device.name().unwrap_or("unknown").to_string();

            std::thread::spawn(move || {
                log::info!("Listening on keyboard: {}", name);
                loop {
                    let events = match device.fetch_events() {
                        Ok(e) => e,
                        Err(err) => {
                            log::error!("evdev read error on {}: {}", name, err);
                            break;
                        }
                    };

                    for event in events {
                        let InputEventKind::Key(key) = event.kind() else { continue };
                        let value = event.value();
                        if value == 2 { continue } // autorepeat

                        let Some(key_name) = canonical(key) else { continue };
                        if value == 0 {
                            if super::MODIFIERS.contains(&key_name) {
                                pressed.lock().unwrap().remove(key_name);
                            }
                            continue;
                        }

                        if super::MODIFIERS.contains(&key_name) {
                            pressed.lock().unwrap().insert(key_name);
                            continue;
                        }

                        let hotkey_str = hotkey.lock().unwrap().clone();
                        if hotkey_str.is_empty() { continue; }

                        let parts: Vec<&str> = hotkey_str.split('+').collect();
                        let Some(trigger_key) = parts.last() else { continue };
                        let modifiers = &parts[..parts.len() - 1];

                        if !key_name.eq_ignore_ascii_case(trigger_key) { continue; }

                        let locked = pressed.lock().unwrap();
                        let all_match = modifiers.iter().all(|m| locked.contains(*m));
                        let exact = locked.iter().filter(|m| super::MODIFIERS.contains(*m)).count() == modifiers.len();

                        if all_match && exact {
                            on_trigger();
                        }
                    }
                }
            });
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    use rdev::{listen, Event, EventType, Key};
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};

    fn canonical(key: &Key) -> Option<&'static str> {
        use Key::*;
        Some(match key {
            ControlLeft | ControlRight => "Control",
            ShiftLeft | ShiftRight => "Shift",
            Alt | AltGr => "Alt",
            MetaLeft | MetaRight => "Meta",

            KeyA => "a", KeyB => "b", KeyC => "c", KeyD => "d",
            KeyE => "e", KeyF => "f", KeyG => "g", KeyH => "h",
            KeyI => "i", KeyJ => "j", KeyK => "k", KeyL => "l",
            KeyM => "m", KeyN => "n", KeyO => "o", KeyP => "p",
            KeyQ => "q", KeyR => "r", KeyS => "s", KeyT => "t",
            KeyU => "u", KeyV => "v", KeyW => "w", KeyX => "x",
            KeyY => "y", KeyZ => "z",

            Num0 => "0", Num1 => "1", Num2 => "2", Num3 => "3",
            Num4 => "4", Num5 => "5", Num6 => "6", Num7 => "7",
            Num8 => "8", Num9 => "9",

            F1 => "F1",   F2 => "F2",   F3 => "F3",   F4 => "F4",
            F5 => "F5",   F6 => "F6",   F7 => "F7",   F8 => "F8",
            F9 => "F9",   F10 => "F10", F11 => "F11", F12 => "F12",

            Tab => "Tab", Return => "Enter", Escape => "Escape",
            Backspace => "Backspace", CapsLock => "CapsLock", Space => "Space",
            Delete => "Delete", Insert => "Insert", Home => "Home", End => "End",
            PageUp => "PageUp", PageDown => "PageDown",
            UpArrow => "ArrowUp", DownArrow => "ArrowDown",
            LeftArrow => "ArrowLeft", RightArrow => "ArrowRight",

            _ => return None,
        })
    }

    pub fn spawn(hotkey: Arc<Mutex<String>>, on_trigger: impl Fn() + Send + Sync + 'static) {
        std::thread::spawn(move || {
            log::info!("Starting global key listener (rdev)");
            let pressed: Mutex<HashSet<&'static str>> = Mutex::new(HashSet::new());

            if let Err(e) = listen(move |event: Event| {
                match event.event_type {
                    EventType::KeyPress(ref key) => {
                        let Some(name) = canonical(key) else { return };
                        if super::MODIFIERS.contains(&name) {
                            pressed.lock().unwrap().insert(name);
                            return;
                        }

                        let hotkey_str = hotkey.lock().unwrap().clone();
                        if hotkey_str.is_empty() { return; }

                        let parts: Vec<&str> = hotkey_str.split('+').collect();
                        let trigger_key = match parts.last() { Some(k) => *k, None => return };
                        let modifiers = &parts[..parts.len() - 1];

                        if !name.eq_ignore_ascii_case(trigger_key) { return; }

                        let locked = pressed.lock().unwrap();
                        let all_match = modifiers.iter().all(|m| locked.contains(*m));
                        let exact = locked.iter().filter(|m| super::MODIFIERS.contains(*m)).count() == modifiers.len();

                        if all_match && exact { on_trigger(); }
                    }
                    EventType::KeyRelease(ref key) => {
                        let Some(name) = canonical(key) else { return };
                        if super::MODIFIERS.contains(&name) {
                            pressed.lock().unwrap().remove(name);
                        }
                    }
                    _ => {}
                }
            }) {
                log::error!("Key listener error: {:?}", e);
            }
        });
    }
}

pub fn spawn(hotkey: Arc<Mutex<String>>, on_trigger: impl Fn() + Send + Sync + 'static) {
    platform::spawn(hotkey, on_trigger);
}
