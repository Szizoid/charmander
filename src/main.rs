use std::time;

use evdev::InputEvent;
use iced_layershell::Settings;
use iced_layershell::build_pattern::application;
use iced_layershell::settings::{LayerShellSettings, StartMode};

use crate::input::evdev::{KeyEventState, find_physical_keyboards, get_parsed_key_events};
use crate::input::uinput::{build_virtual_keyboard, create_event};
use crate::state::machine::{Emulate, State};
use crate::ui::window::{Charmander, namespace, update, view};

mod input;
mod state;
mod ui;

fn main() -> Result<(), iced_layershell::Error> {
    let _ = application(Charmander::default, namespace, update, view)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((0, 400)),
                exclusive_zone: 400,
                start_mode: StartMode::Active,
                ..Default::default()
            },
            ..Default::default()
        })
        .run();
    Ok(())
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
