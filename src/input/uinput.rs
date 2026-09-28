use std::io::Result;

use evdev::{AttributeSet, EventType, InputEvent, KeyCode, uinput::VirtualDevice};

use crate::config;
use crate::input::evdev::KeyEventState;

const KEY_CODES: [KeyCode; 103] = [
    // letters
    KeyCode::KEY_A,
    KeyCode::KEY_B,
    KeyCode::KEY_C,
    KeyCode::KEY_D,
    KeyCode::KEY_E,
    KeyCode::KEY_F,
    KeyCode::KEY_G,
    KeyCode::KEY_H,
    KeyCode::KEY_I,
    KeyCode::KEY_J,
    KeyCode::KEY_K,
    KeyCode::KEY_L,
    KeyCode::KEY_M,
    KeyCode::KEY_N,
    KeyCode::KEY_O,
    KeyCode::KEY_P,
    KeyCode::KEY_Q,
    KeyCode::KEY_R,
    KeyCode::KEY_S,
    KeyCode::KEY_T,
    KeyCode::KEY_U,
    KeyCode::KEY_V,
    KeyCode::KEY_W,
    KeyCode::KEY_X,
    KeyCode::KEY_Y,
    KeyCode::KEY_Z,
    // digits row
    KeyCode::KEY_1,
    KeyCode::KEY_2,
    KeyCode::KEY_3,
    KeyCode::KEY_4,
    KeyCode::KEY_5,
    KeyCode::KEY_6,
    KeyCode::KEY_7,
    KeyCode::KEY_8,
    KeyCode::KEY_9,
    KeyCode::KEY_0,
    KeyCode::KEY_MINUS,
    KeyCode::KEY_EQUAL,
    // punctuation
    KeyCode::KEY_LEFTBRACE,
    KeyCode::KEY_RIGHTBRACE,
    KeyCode::KEY_SEMICOLON,
    KeyCode::KEY_APOSTROPHE,
    KeyCode::KEY_GRAVE,
    KeyCode::KEY_BACKSLASH,
    KeyCode::KEY_COMMA,
    KeyCode::KEY_DOT,
    KeyCode::KEY_SLASH,
    // whitespace / editing
    KeyCode::KEY_ESC,
    KeyCode::KEY_TAB,
    KeyCode::KEY_ENTER,
    KeyCode::KEY_BACKSPACE,
    KeyCode::KEY_SPACE,
    KeyCode::KEY_CAPSLOCK,
    KeyCode::KEY_INSERT,
    KeyCode::KEY_DELETE,
    // modifiers
    KeyCode::KEY_LEFTCTRL,
    KeyCode::KEY_RIGHTCTRL,
    KeyCode::KEY_LEFTSHIFT,
    KeyCode::KEY_RIGHTSHIFT,
    KeyCode::KEY_LEFTALT,
    KeyCode::KEY_RIGHTALT,
    KeyCode::KEY_LEFTMETA,
    KeyCode::KEY_RIGHTMETA,
    // navigation
    KeyCode::KEY_HOME,
    KeyCode::KEY_END,
    KeyCode::KEY_PAGEUP,
    KeyCode::KEY_PAGEDOWN,
    KeyCode::KEY_UP,
    KeyCode::KEY_DOWN,
    KeyCode::KEY_LEFT,
    KeyCode::KEY_RIGHT,
    // function keys
    KeyCode::KEY_F1,
    KeyCode::KEY_F2,
    KeyCode::KEY_F3,
    KeyCode::KEY_F4,
    KeyCode::KEY_F5,
    KeyCode::KEY_F6,
    KeyCode::KEY_F7,
    KeyCode::KEY_F8,
    KeyCode::KEY_F9,
    KeyCode::KEY_F10,
    KeyCode::KEY_F11,
    KeyCode::KEY_F12,
    // lock keys / misc
    KeyCode::KEY_NUMLOCK,
    KeyCode::KEY_SCROLLLOCK,
    KeyCode::KEY_PAUSE,
    KeyCode::KEY_SYSRQ,
    // numpad
    KeyCode::KEY_KP0,
    KeyCode::KEY_KP1,
    KeyCode::KEY_KP2,
    KeyCode::KEY_KP3,
    KeyCode::KEY_KP4,
    KeyCode::KEY_KP5,
    KeyCode::KEY_KP6,
    KeyCode::KEY_KP7,
    KeyCode::KEY_KP8,
    KeyCode::KEY_KP9,
    KeyCode::KEY_KPDOT,
    KeyCode::KEY_KPPLUS,
    KeyCode::KEY_KPMINUS,
    KeyCode::KEY_KPASTERISK,
    KeyCode::KEY_KPSLASH,
    KeyCode::KEY_KPENTER,
];

/// # Errors
///
/// Returns an error if `/dev/uinput` can't be opened (missing permissions or
/// the `uinput` module isn't loaded), or if the kernel refuses to create the device.
pub fn build_virtual_keyboard() -> Result<VirtualDevice> {
    let mut builder = VirtualDevice::builder()?;
    builder = builder.name(config::APP_NAME);
    let keys = AttributeSet::from_iter(KEY_CODES);
    builder = builder.with_keys(&keys)?;
    builder.build()
}

#[must_use]
pub fn create_event(keys: &[KeyCode], state: KeyEventState) -> Vec<InputEvent> {
    let mut events: Vec<InputEvent> = Vec::new();
    for key in keys {
        let event = InputEvent::new(EventType::KEY.0, key.0, state.into());
        events.push(event);
    }
    events
}
