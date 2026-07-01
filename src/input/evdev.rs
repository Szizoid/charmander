use std::path::PathBuf;

use evdev::{Device, KeyCode};

pub fn find_physical_keyboards() -> Vec<(PathBuf, Device)> {
    let mut found: Vec<(PathBuf, Device)> = Vec::new();

    for (path, device) in evdev::enumerate() {
        let has_enter = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(KeyCode::KEY_ENTER));
        let has_physical_path = device.physical_path().is_some();

        if has_enter && has_physical_path {
            founded.push((path, device));
        }
    }
    found
}
