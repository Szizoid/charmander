use std::sync::mpsc;
use std::thread;

use iced_layershell::Settings;
use iced_layershell::build_pattern::application;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, StartMode};

use charmander::config;
use charmander::input::evdev::{KeyEventState, find_physical_keyboards, get_parsed_key_events};
use charmander::input::uinput::{build_virtual_keyboard, create_event};
use charmander::state::{Emulate, State};
use charmander::ui::window::{Charmander, namespace, subscription, update, view};

fn main() -> Result<(), iced_layershell::Error> {
    let (_tx, _rx) = mpsc::channel::<State>();

    let _kb_thread = thread::spawn(move || {
        let mut keyboard = find_physical_keyboards().unwrap().pop().unwrap().1;
        keyboard.grab().unwrap();

        let mut virtual_keyboard = build_virtual_keyboard().unwrap();

        let mut state = State::default();

        loop {
            let mut to_emulate = Vec::new();
            let events = get_parsed_key_events(&mut keyboard).unwrap();

            for event in events {
                let (code, kind, now) = event;

                let (new_state, action) = state.handle(code, kind, now);
                state = new_state;

                if action.forward {
                    to_emulate.append(&mut create_event(&[code], kind));
                }
                match action.emulate {
                    Emulate::Nothing => {}
                    Emulate::Release(released_key) => {
                        to_emulate
                            .append(&mut create_event(&[released_key], KeyEventState::Release));
                    }
                    Emulate::Commit(value) => println!("would commit: {value}"),
                }
            }

            virtual_keyboard.emit(&to_emulate).unwrap();
        }
    });

    application(Charmander::default, namespace, update, view)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                anchor: Anchor::empty(),
                layer: Layer::Overlay,
                exclusive_zone: config::EXCLUSIVE_ZONE,
                size: config::WINDOW_SIZE,
                keyboard_interactivity: KeyboardInteractivity::None,
                events_transparent: true,
                start_mode: StartMode::Active, // TODO: поменять на daemon + Background
                ..Default::default()
            },
            ..Default::default()
        })
        .subscription(subscription)
        .run()?;

    Ok(())
}
