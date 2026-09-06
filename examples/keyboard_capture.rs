//! Manual check of the keyboard-capture pipeline in isolation from the UI:
//! evdev (grab the physical keyboard) -> state machine -> uinput (virtual keyboard).
//!
//! Run with `cargo run --example keyboard_capture` (needs access to /dev/input/*
//! and /dev/uinput — usually root or membership in the relevant group).
//! Runs for 30 seconds, then exits on its own.

use std::time;

use evdev::InputEvent;

use charmander::input::evdev::{KeyEventState, find_physical_keyboards, get_parsed_key_events};
use charmander::input::uinput::{build_virtual_keyboard, create_event};
use charmander::state::machine::{Emulate, State};

fn main() {
    // Grab whatever physical keyboard we find first (see the TODO in
    // find_physical_keyboards about picking a specific device when there are
    // several candidates) via EVIOCGRAB: from this point on, only this process
    // sees its events — the compositor no longer gets them directly.
    let mut my_keyboard = find_physical_keyboards().unwrap().pop().unwrap().1;
    my_keyboard.grab().unwrap();

    let start_time = time::SystemTime::now();

    // Virtual keyboard via uinput — what we "give back" to the system instead of
    // the grabbed physical one (either forwarding events as-is, or replacing them —
    // see Emulate).
    let mut virtual_keyboard = build_virtual_keyboard().unwrap();

    let mut state = State::Idle;

    // 30-second cap so the test doesn't keep the keyboard grabbed forever if you
    // forget to kill the process by hand.
    while time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs()
        < 30
    {
        // Events that will actually go out to the virtual keyboard this tick.
        let mut false_events: Vec<InputEvent> = Vec::new();

        // Real events coming from the physical keyboard this tick.
        let true_events = get_parsed_key_events(&mut my_keyboard).unwrap();

        for true_event in true_events {
            let (code, kind, now) = true_event;

            // Run each real event through the state machine: it decides whether
            // to forward it as-is and/or replace it with a synthetic one.
            let (new_state, action) = state.handle(code, kind, now);
            state = new_state;

            if action.forward {
                false_events.append(&mut create_event(&[code], kind));
            }
            match action.emulate {
                Emulate::Nothing => {}
                Emulate::Release(released_key) => {
                    // Synthetic Release — the fix for the compositor's autorepeat
                    // (see the note about the Tracking -> Accent transition in
                    // state/machine.rs): the key is still physically held down,
                    // but the system needs to think it's released.
                    false_events.append(&mut create_event(&[released_key], KeyEventState::Release));
                }
                Emulate::Commit(value) => println!("would commit: {value}"),
            }
        }

        // Emit all events accumulated this tick into the virtual keyboard at once.
        virtual_keyboard.emit(&false_events).unwrap();
    }
}
