use crate::input::evdev::{find_physical_keyboards, get_key_events};

mod input;

fn main() {
    let mut devices = find_physical_keyboards().unwrap();
    let (_, mut device) = devices.pop().unwrap();
    let events = get_key_events(&mut device).unwrap();
    for event in events {
        println!("{:?}", event);
    }
}
