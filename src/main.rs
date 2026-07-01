mod input;

fn main() {
    let mut devices = input::evdev::find_physical_keyboards();
    let (_, mut device) = devices.pop().unwrap();
    loop {
        let events = device.fetch_events().unwrap();
        for event in events {
            println!("{:?}", event);
        }
    }
}
