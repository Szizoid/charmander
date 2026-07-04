use std::time;

use evdev::InputEvent;

use crate::input::{
    evdev::{find_physical_keyboards, get_parsed_key_events},
    uinput::{build_virtual_keyboard, create_event},
};

mod input;

fn main() {
    let mut my_keyboard = find_physical_keyboards().unwrap().pop().unwrap().1;
    my_keyboard.grab();
    let start_time = time::SystemTime::now();
    let mut virtual_keyboard = build_virtual_keyboard().unwrap();
    while time::SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_secs()
        < 30
    {
        let mut false_events: Vec<InputEvent> = Vec::new();
        let true_events = get_parsed_key_events(&mut my_keyboard).unwrap();
        for true_event in true_events {
            let (code, state, _) = true_event;
            false_events.append(&mut create_event(&[code], state));
        }
        virtual_keyboard.emit(&false_events);
    }
}
