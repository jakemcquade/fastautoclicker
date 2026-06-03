use crate::inputs::MouseButton;
use std::{
    fs::{self, File},
    sync::{Mutex, OnceLock},
    thread,
    time::{Duration, SystemTime},
};
use input_linux::{
    EventKind, EventTime, InputEvent, InputId, Key, KeyEvent, KeyState, SynchronizeEvent,
    UInputHandle,
    sys::{BUS_USB, input_event},
};

static DEVICE: OnceLock<Mutex<UInputHandle<File>>> = OnceLock::new();

fn button_to_key(button: MouseButton) -> Key {
    match button {
        MouseButton::Left => Key::ButtonLeft,
        MouseButton::Middle => Key::ButtonMiddle,
        MouseButton::Right => Key::ButtonRight,
    }
}

fn now() -> EventTime {
    let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    EventTime::new(t.as_secs() as i64, t.subsec_micros() as i64)
}

fn device() -> Result<&'static Mutex<UInputHandle<File>>, String> {
    if let Some(d) = DEVICE.get() {
        return Ok(d);
    }

    // Open uinput. Needs write access (run as root or be in the `input` group).
    let file = fs::OpenOptions::new()
        .write(true)
        .open("/dev/uinput")
        .map_err(|e| format!("cannot open /dev/uinput (try as root): {e}"))?;

    let handle = UInputHandle::new(file);

    handle.set_evbit(EventKind::Key).map_err(|e| e.to_string())?;
    handle
        .set_evbit(EventKind::Synchronize)
        .map_err(|e| e.to_string())?;
    // Register every button we might send.
    for key in [Key::ButtonLeft, Key::ButtonMiddle, Key::ButtonRight] {
        handle.set_keybit(key).map_err(|e| e.to_string())?;
    }

    handle
        .create(
            &InputId {
                bustype: BUS_USB,
                vendor: 0x3232,
                product: 0x5678,
                version: 0x1234,
            },
            b"autoclicker",
            0,
            &[],
        )
        .map_err(|e| e.to_string())?;

    // Give the system a moment to register the device before the first click.
    thread::sleep(Duration::from_millis(500));

    Ok(DEVICE.get_or_init(|| Mutex::new(handle)))
}

/// Press and release the given mouse button once.
pub fn send(button: MouseButton) -> Result<(), String> {
    let key = button_to_key(button);
    let handle = device()?.lock().map_err(|e| e.to_string())?;
    send_key(&handle, key, KeyState::PRESSED)?;
    send_key(&handle, key, KeyState::RELEASED)?;
    Ok(())
}

fn send_key(handle: &UInputHandle<File>, key: Key, state: KeyState) -> Result<(), String> {
    let events: [input_event; 2] = [
        InputEvent::from(KeyEvent::new(now(), key, state))
            .as_raw()
            .to_owned(),
        InputEvent::from(SynchronizeEvent::report(now()))
            .as_raw()
            .to_owned(),
    ];
    handle.write(&events).map_err(|e| e.to_string())?;
    Ok(())
}