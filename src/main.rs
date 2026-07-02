use crate::input::evdev::get_events;

mod input;

fn main() {
    let mut devices = input::evdev::find_physical_keyboards().unwrap();
    let (_, mut device) = devices.pop().unwrap();
    loop {
        let events = get_events(&mut device).unwrap();
        for event in events {
            dbg!(event);
        }
    }
}
