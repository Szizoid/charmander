use evdev::KeyCode;
use std::time::Duration;

use crate::state::machine::Candidate;

pub const APP_NAME: &str = "Charmander";

// STATE MACHINE settings
pub const TRACKING_TIME_MS: Duration = Duration::from_millis(1000);
pub const MOD_KEY: KeyCode = KeyCode::KEY_LEFT;
pub const EXIT_KEY: KeyCode = KeyCode::KEY_ESC;

pub fn _symbols() -> Vec<Candidate> {
    // Candidate-parsing logic from some external file goes here. Might make sense
    // to move this into a separate submodule.
    todo!();
}

pub fn test_symbols() -> Vec<Candidate> {
    vec![Candidate {
        name: String::from("test1"),
        value: String::from("test_value1"),
    }]
}

// UI settings
pub const WINDOW_SIZE: Option<(u32, u32)> = Some((0, 400));
pub const EXCLUSIVE_ZONE: i32 = 400;
pub const SELECTED_COLOR: (f32, f32, f32) = (1.0, 1.0, 0.0);
pub const SELECTED_COLOR_RED: f32 = SELECTED_COLOR.0;
pub const SELECTED_COLOR_GREEN: f32 = SELECTED_COLOR.1;
pub const SELECTED_COLOR_BLUE: f32 = SELECTED_COLOR.2;

// INPUT settings
pub const PROBE_KEY: KeyCode = KeyCode::KEY_ENTER;
