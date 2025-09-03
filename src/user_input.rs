use minifb::Window;

use crate::constants::KEYBOARD_CODES;

/// Checks the current state of the keyboard and returns the first pressed key's corresponding hex code.
pub fn get_pressed_key(window: &Window) -> Option<u8> {
    for (key, hex_code) in KEYBOARD_CODES.iter() {
        if window.is_key_down(*key) {
            return Some(*hex_code);
        }
    }

    None
}