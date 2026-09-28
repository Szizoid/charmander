use evdev::KeyCode;
use iced::Color;
use std::time::Duration;

use crate::state::Candidate;

pub const APP_NAME: &str = "Charmander";

// STATE MACHINE settings
pub const TRACKING_TIME_MS: Duration = Duration::from_millis(1000);
pub const MOD_KEY: KeyCode = KeyCode::KEY_LEFT;
pub const EXIT_KEY: KeyCode = KeyCode::KEY_ESC;

#[must_use]
pub fn _symbols() -> Vec<Candidate> {
    // Candidate-parsing logic from some external file goes here. Might make sense
    // to move this into a separate submodule.
    todo!();
}

#[allow(clippy::must_use_candidate)]
pub fn test_symbols() -> Vec<Candidate> {
    vec![
        Candidate {
            name: String::from("l with stroke"),
            value: String::from("ł"),
        },
        Candidate {
            name: String::from("l with acute"),
            value: String::from("ĺ"),
        },
        Candidate {
            name: String::from("l with caron"),
            value: String::from("ľ"),
        },
        Candidate {
            name: String::from("l with cedilla"),
            value: String::from("ļ"),
        },
        Candidate {
            name: String::from("l with middle dot"),
            value: String::from("ŀ"),
        },
    ]
}

// UI settings
pub const WINDOW_SIZE: Option<(u32, u32)> = Some((0, 400));
pub const EXCLUSIVE_ZONE: i32 = 400;
pub const SELECTED_COLOR: Color = Color::from_rgb(1.0, 1.0, 0.0);

// INPUT settings
pub const PROBE_KEY: KeyCode = KeyCode::KEY_ENTER;
