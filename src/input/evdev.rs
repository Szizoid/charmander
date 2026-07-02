use std::path::PathBuf;

use evdev::{Device, FetchEventsSynced, InputEvent, KeyCode};

pub fn find_physical_keyboards() -> Option<Vec<(PathBuf, Device)>> {
    let mut found: Vec<(PathBuf, Device)> = Vec::new();

    for (path, device) in evdev::enumerate() {
        let has_enter = device
            .supported_keys()
            .is_some_and(|keys| keys.contains(KeyCode::KEY_ENTER));
        let has_physical_path = device.physical_path().is_some();

        if has_enter && has_physical_path {
            found.push((path, device));
        }
    }
    if found.iter().len() > 0 {
        Some(found)
    } else {
        Option::None
    }
}

pub fn get_events(device: &mut Device) -> Option<FetchEventsSynced> {
    let events = device.fetch_events();
    match events {
        Result::Ok(ev) => Some(ev),
        Result::Err(_) => Option::None,
    }
}
