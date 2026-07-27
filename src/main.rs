use std::time;

use evdev::InputEvent;

use crate::{
    input::{
        evdev::{KeyEventState, find_physical_keyboards, get_parsed_key_events},
        uinput::{build_virtual_keyboard, create_event},
    },
    state::machine::{Emulate, State},
};

mod input;
mod state;

fn main() {
    // waycrate/exwlshelleventloop counter example
}

#[allow(unused)]
fn test_state_machine() {
    let mut my_keyboard = find_physical_keyboards().unwrap().pop().unwrap().1;
    my_keyboard.grab().unwrap();
    let start_time = time::SystemTime::now();
    let mut virtual_keyboard = build_virtual_keyboard().unwrap();
    let mut state = State::Idle;
    while time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs()
        < 30
    {
        let mut false_events: Vec<InputEvent> = Vec::new();
        let true_events = get_parsed_key_events(&mut my_keyboard).unwrap();
        for true_event in true_events {
            let (code, kind, now) = true_event;
            let (new_state, action) = state.handle(code, kind, now);
            state = new_state;

            if action.forward {
                false_events.append(&mut create_event(&[code], kind));
            }
            match action.emulate {
                Emulate::Nothing => {}
                Emulate::Release(released_key) => {
                    false_events.append(&mut create_event(&[released_key], KeyEventState::Release));
                }
                Emulate::Commit(value) => println!("would commit: {value}"),
            }
        }
        virtual_keyboard.emit(&false_events).unwrap();
    }
}
