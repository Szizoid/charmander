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

pub enum KeyEventState {
    Release = 0,
    Press = 1,
    Repeat = 2,
}

impl TryFrom<i32> for KeyEventState {
    type Error = io::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyEventState::Release),
            1 => Ok(KeyEventState::Press),
            2 => Ok(KeyEventState::Repeat),
            _ => Err(io::Error::other(format!(
                "unexpected value for key state: {}",
                value
            ))),
        }
    }
}

pub fn get_parsed_key_events(
    device: &mut Device,
) -> Result<Vec<(KeyCode, KeyEventState, SystemTime)>, io::Error> {
    let mut parsed_events: Vec<(KeyCode, KeyEventState, SystemTime)> = Vec::new();
    let events = device.fetch_events()?;
    for event in events {
        if event.event_type() == EventType::KEY {
            let EventSummary::Key(_, code, raw_state) = event.destructure() else {
                continue;
            };
            let state = KeyEventState::try_from(raw_state)?;
            parsed_events.push((code, state, event.timestamp()));
        }
    }
    Ok(parsed_events)
}
