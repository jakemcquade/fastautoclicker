use std::sync::{mpsc, Arc, Mutex};
use tauri::{Emitter, Manager};

use crate::modules;

pub struct AppState {
    pub status: Mutex<bool>,
    pub interval: Mutex<u64>,
    pub hotkey: Arc<Mutex<String>>,
    pub hotkey_recording: Mutex<bool>,
    pub click_type: Mutex<u8>,
    pub mouse_button: Mutex<u8>,
    pub click_location: Mutex<Option<(i32, i32)>>,
    pub repeat_count: Mutex<u64>,
    pub stop_interval: Mutex<Option<mpsc::Sender<()>>>,
}

impl AppState {
    pub fn default() -> Self {
        Self {
            status: Mutex::new(false),
            interval: Mutex::new(100),
            hotkey: Arc::new(Mutex::new("F6".to_string())),
            hotkey_recording: Mutex::new(false),
            click_type: Mutex::new(0),
            mouse_button: Mutex::new(0),
            click_location: Mutex::new(None),
            repeat_count: Mutex::new(0),
            stop_interval: Mutex::new(None),
        }
    }

    pub fn toggle_click_loop(&self, app: tauri::AppHandle) -> bool {
        let mut stop_interval = self.stop_interval.lock().unwrap();
        let mut status = self.status.lock().unwrap();
        let location = self.click_location.lock().unwrap().clone();
        *status = !*status;

        if *status {
            let (tx, rx) = mpsc::channel();
            let interval = std::time::Duration::from_millis(
                modules::util::sanitize_interval(*self.interval.lock().unwrap()),
            );

            let mtype = crate::inputs::MouseButton::from(*self.mouse_button.lock().unwrap());
            let ctype = crate::inputs::ClickType::from(*self.click_type.lock().unwrap());
            let repeat = *self.repeat_count.lock().unwrap();

            let clicks = match ctype {
                crate::inputs::ClickType::Single => 1,
                crate::inputs::ClickType::Double => 2,
            };

            if let Err(e) = app.emit("state", true) {
                log::warn!("Failed to emit state event: {e}");
            }

            let app = app.clone();
            std::thread::spawn(move || {
                let mut remaining = repeat;

                loop {
                    for _ in 0..clicks {
                        modules::util::send_click(mtype, location);
                    }

                    if repeat != 0 {
                        remaining -= 1;
                        if remaining == 0 {
                            {
                                let state = app.state::<AppState>();
                                let mut stop_interval = state.stop_interval.lock().unwrap();
                                let mut status = state.status.lock().unwrap();
                                *status = false;
                                *stop_interval = None;
                            }

                            if let Err(e) = app.emit("state", false) {
                                log::warn!("Failed to emit state event: {e}");
                            }
                            break;
                        }
                    }

                    match rx.recv_timeout(interval) {
                        Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                    }
                }
            });

            *stop_interval = Some(tx);
        } else {
            if let Some(tx) = stop_interval.take() {
                let _ = tx.send(());
            }

            if let Err(e) = app.emit("state", false) {
                log::warn!("Failed to emit state event: {e}");
            }
        }

        log::info!("State: {}", if *status { "Enabled" } else { "Disabled" });
        *status
    }
}
