use std::{io, path::PathBuf, time::SystemTime};

use evdev::{Device, EventSummary, EventType, InputEvent, KeyCode};

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
    if found.is_empty() {
        Option::None
    } else {
        Some(found)
    }
}

pub fn get_key_events(device: &mut Device) -> Result<Vec<InputEvent>, io::Error> {
    let mut key_events: Vec<InputEvent> = Vec::new();
    let events = device.fetch_events()?;
    for event in events {
        if event.event_type() == EventType::KEY {
            key_events.push(event);
        }
    }

    Ok(key_events)
}

pub fn parse_event(event: InputEvent) -> Option<(KeyCode, i32, SystemTime)> {
    let timestamp = event.timestamp();
    if let EventSummary::Key(_, key_code, state) = event.destructure() {
        Some((key_code, state, timestamp))
    } else {
        Option::None
    }
}
