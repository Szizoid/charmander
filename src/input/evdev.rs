use std::{io, path::PathBuf, time::SystemTime};

use evdev::{Device, EventSummary, EventType, KeyCode};

// TODO: when this finds more than one physical keyboard, add a way to pick a specific
// one instead of grabbing an arbitrary one (config file, env var, or an interactive
// CLI/TUI prompt listing the candidates).
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

pub fn get_parsed_key_events(
    device: &mut Device,
) -> Result<Vec<(KeyCode, i32, SystemTime)>, io::Error> {
    let mut parsed_events: Vec<(KeyCode, i32, SystemTime)> = Vec::new();
    let events = device.fetch_events()?;
    for event in events {
        if event.event_type() == EventType::KEY {
            let EventSummary::Key(_, code, state) = event.destructure() else {
                continue;
            };
            parsed_events.push((code, state, event.timestamp()));
        }
    }
    Ok(parsed_events)
}
