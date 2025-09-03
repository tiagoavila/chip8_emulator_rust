use minifb::{Window, WindowOptions};

use crate::{chip8, constants::{SCREEN_HEIGHT, SCREEN_SCALE_FACTOR, SCREEN_WIDTH}};

/// Initializes and returns a new window for the Chip-8 emulator.
pub fn initialize_window() -> Window {
    let mut window = Window::new(
        "Chip-8 Emulator _ Use Esc to exit",
        SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
        SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    window
}

/// Prints debug information about the current state of the Chip-8 emulator.
/// Only draw if `chip8.needs_redraw` is true.
pub fn draw_screen_if_needed(buffer: &mut Vec<u32>, chip8: &chip8::Chip8) {
    if chip8.needs_redraw {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel_on = chip8.screen[y * SCREEN_WIDTH + x];
                let color = if pixel_on { 0xFFFFFF } else { 0x000000 };
                // Draw a block of size SCREEN_SCALE_FACTOR x SCREEN_SCALE_FACTOR
                for dy in 0..SCREEN_SCALE_FACTOR {
                    for dx in 0..SCREEN_SCALE_FACTOR {
                        let scaled_x = x * SCREEN_SCALE_FACTOR + dx;
                        let scaled_y = y * SCREEN_SCALE_FACTOR + dy;
                        let buffer_index =
                            scaled_y * (SCREEN_WIDTH * SCREEN_SCALE_FACTOR) + scaled_x;
                        buffer[buffer_index] = color;
                    }
                }
            }
        }
    }
}

/// Updates the window with the current buffer content.
pub fn update_window_with_buffer(buffer: &mut Vec<u32>, window: &mut Window) {
    window
        .update_with_buffer(
            &buffer,
            SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
            SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
        )
        .unwrap();
}

/// Initializes and returns a new buffer for the Chip-8 emulator.
pub fn initialize_buffer() -> Vec<u32> {
    vec![0; SCREEN_WIDTH * SCREEN_SCALE_FACTOR * SCREEN_HEIGHT * SCREEN_SCALE_FACTOR]
}