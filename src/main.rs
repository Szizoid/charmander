mod input;

fn main() {
    let devices = input::evdev::find_physical_keyboards();
    for (path, device) in devices {
        println!("{:?}: {}", path, device.name().unwrap());
    }
}
