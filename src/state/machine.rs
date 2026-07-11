use std::time::SystemTime;

use evdev::KeyCode;

use crate::input::evdev::KeyEventState;

pub mod config {
    use evdev::KeyCode;
    use std::time::Duration;

    pub const TRACKING_TIME_MS: Duration = Duration::from_millis(1000);
    pub const MOD_KEY: KeyCode = KeyCode::KEY_LEFT;
    pub const EXIT_KEY: KeyCode = KeyCode::KEY_ESC;
}

#[derive(Debug)]
pub enum State {
    Idle,
    Tracking {
        tracking_key: KeyCode,
        pressed_at: SystemTime,
    },
    Accent {
        target_key: KeyCode,
        symbols: Vec<Candidate>,
        selection: usize,
    },
}

impl State {
    pub fn handle(
        self,
        key: KeyCode,
        event_state: KeyEventState,
        now: SystemTime,
    ) -> (State, Action) {
        match &self {
            State::Idle => match event_state {
                KeyEventState::Press if key != config::MOD_KEY && key != config::EXIT_KEY => (
                    State::Tracking {
                        tracking_key: key,
                        pressed_at: now,
                    },
                    Action::forward(),
                ),
                _ => (self, Action::forward()),
            },
            State::Tracking {
                tracking_key,
                pressed_at,
            } => {
                let tracking_key = *tracking_key;
                let pressed_at = *pressed_at;
                match (key, event_state) {
                    // Mod + Pressed
                    (config::MOD_KEY, KeyEventState::Press) => {
                        if now
                            .duration_since(pressed_at)
                            .expect(
                                "SystemTime is compared to SystemTime. There should be no errors",
                            )
                            .le(&config::TRACKING_TIME_MS)
                        {
                            (
                                State::Accent {
                                    target_key: tracking_key,
                                    symbols: vec![Candidate {
                                        name: String::from("test1"),
                                        value: String::from("test_value1"),
                                    }],
                                    selection: 0,
                                },
                                Action::suppress(),
                            )
                        } else {
                            (State::Idle, Action::forward())
                        }
                    }
                    // Tracking + Released
                    (compared_key, KeyEventState::Release) if compared_key == tracking_key => {
                        (State::Idle, Action::forward())
                    }
                    // Tracking + Pressed
                    // Other + Pressed
                    (_, KeyEventState::Press) => (
                        State::Tracking {
                            tracking_key: key,
                            pressed_at: now,
                        },
                        Action::forward(),
                    ),
                    // Mod + Released
                    // Mod + Repeated
                    // Tracking + Repeated
                    // Other + Released
                    // Other + Repeated
                    _ => (self, Action::forward()),
                }
            }
            State::Accent {
                target_key,
                symbols,
                selection,
            } => {
                let target_key = *target_key;
                let selection = *selection;
                match (key, event_state) {
                    // Mod + Pressed
                    // Mod + Repeated
                    (config::MOD_KEY, KeyEventState::Press | KeyEventState::Repeat) => (
                        State::Accent {
                            target_key,
                            selection: (selection + 1) % symbols.len(),
                            symbols: symbols.clone(),
                        },
                        Action::suppress(),
                    ),
                    // Exit + Pressed
                    (config::EXIT_KEY, KeyEventState::Press) => (State::Idle, Action::suppress()),
                    // Target + Released
                    (compared_key, KeyEventState::Release) if compared_key == target_key => (
                        State::Idle,
                        Action {
                            forward: true,
                            emulate: Emulate::Commit(symbols[selection].value.clone()),
                        },
                    ),
                    // Target + Pressed
                    // Other + Pressed
                    (_, KeyEventState::Press) => (
                        State::Tracking {
                            tracking_key: key,
                            pressed_at: now,
                        },
                        Action::forward(),
                    ),
                    // Mod + Released
                    // Exit + Released
                    // Other + Released
                    (compared_key, KeyEventState::Release) if compared_key != target_key => {
                        (self, Action::forward())
                    }
                    // Exit + Repeated
                    // Target + Repeated
                    // Other + Repeated
                    _ => (self, Action::suppress()),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub name: String,
    pub value: String,
}

pub struct Action {
    pub forward: bool,
    pub emulate: Emulate,
}

impl Action {
    pub fn forward() -> Self {
        Action {
            forward: true,
            emulate: Emulate::Nothing,
        }
    }

    pub fn suppress() -> Self {
        Action {
            forward: false,
            emulate: Emulate::Nothing,
        }
    }
}

pub enum Emulate {
    Nothing,
    Commit(String),
}
