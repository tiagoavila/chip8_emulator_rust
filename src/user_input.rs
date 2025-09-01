use minifb::Window;

use crate::constants::KEYBOARD_CODES;

pub fn get_pressed_key(window: &Window) -> Option<u8> {
    for (key, hex_code) in KEYBOARD_CODES.iter() {
        if window.is_key_down(*key) {
            return Some(*hex_code);
        }
    }

    None
}

pub fn check_released_keys(window: &Window, chip8_keyboard: &mut [bool; 16]) {
    for (key, hex_code) in KEYBOARD_CODES.iter() {
        if window.is_key_released(*key) {
            println!("Key released: {:#X}", hex_code);
            chip8_keyboard[*hex_code as usize] = false;
        }
    }
}