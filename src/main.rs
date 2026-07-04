use evdev::EventSummary;

use crate::input::evdev::{find_physical_keyboards, get_key_events, parse_event};

mod input;

fn main() {
    let mut devices = find_physical_keyboards().unwrap();
    let (_, mut device) = devices.pop().unwrap();
    let events = get_key_events(&mut device).unwrap();
    for event in events {
        let (code, state, time) = parse_event(event).expect("Это точно KeyEvent");
    }
}
