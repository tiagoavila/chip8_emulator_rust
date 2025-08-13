use std::{
    fmt::format,
    fs::File,
    io::{self, BufRead, Read},
};

use minifb::{Key, Window, WindowOptions};

use crate::constants::{SCREEN_HEIGHT, SCREEN_SCALE_FACTOR, SCREEN_WIDTH};

mod chip8;
mod constants;

fn main() {
    let mut buffer: Vec<u32> =
        vec![0; SCREEN_WIDTH * SCREEN_SCALE_FACTOR * SCREEN_HEIGHT * SCREEN_SCALE_FACTOR];
    let mut chip8 = chip8::Chip8::new();
    let binary = read_rom("files/roms/IBM_Logo.ch8").unwrap();
    chip8.memory[0x200..0x200 + binary.len()].copy_from_slice(&binary);

    // 00e0
    chip8.tick();
    // a22a
    chip8.tick();
    // 600c
    chip8.tick();
    // 6108
    chip8.tick();
    // d01f
    chip8.tick();

    let mut window = Window::new(
        "Test - ESC to exit",
        SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
        SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear buffer
        buffer.fill(0x000000);

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel_on = chip8.screen[y * SCREEN_WIDTH + x];
                let color = if pixel_on { 0xFFFFFF } else { 0x000000 };
                // Draw a block of size SCREEN_SCALE_FACTOR x SCREEN_SCALE_FACTOR
                for dy in 0..SCREEN_SCALE_FACTOR {
                    for dx in 0..SCREEN_SCALE_FACTOR {
                        let scaled_x = x * SCREEN_SCALE_FACTOR + dx;
                        let scaled_y = y * SCREEN_SCALE_FACTOR + dy;
                        let buffer_index = scaled_y * (SCREEN_WIDTH * SCREEN_SCALE_FACTOR) + scaled_x;
                        buffer[buffer_index] = color;
                    }
                }
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(
                &buffer,
                SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
                SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
            )
            .unwrap();
    }
    //
    // println!("Program is running...");
    // println!("Press Enter to exit...");
    //
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn read_rom(file_path: &str) -> io::Result<Vec<u8>> {
    // Open the file
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer)?;
    // Collect the lines into a vector
    // let lines: Vec<String> = buffer.iter().collect();
    Ok(buffer)
}
