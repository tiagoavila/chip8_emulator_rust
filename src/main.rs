use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use minifb::{Key, Window, WindowOptions};

use crate::constants::{SCREEN_HEIGHT, SCREEN_SCALE_FACTOR, SCREEN_WIDTH};

mod chip8;
mod constants;

/// Command-line arguments for the Chip-8 Emulator
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "A Chip-8 Emulator written in Rust")]
struct Args {
    /// Run the emulator in debug mode
    #[arg(long)]
    debug: bool,

    /// Number of instructions to execute before manual stepping in debug mode
    #[arg(long = "instruction_count", default_value_t = 20)]
    instruction_count: usize,
}

/// Example usage:
/// Normal mode: `cargo run`
/// Debug mode: `cargo run -- --debug --instruction_count 50`

fn main() {
    let args = Args::parse();
    let debug_mode = args.debug;
    let instruction_count = args.instruction_count;

    let mut buffer: Vec<u32> =
        vec![0; SCREEN_WIDTH * SCREEN_SCALE_FACTOR * SCREEN_HEIGHT * SCREEN_SCALE_FACTOR];

    // let binary = read_rom("files/roms/IBM_Logo.ch8").unwrap();
    // let binary = read_rom("files/roms/chip8-logo.ch8").unwrap();
    let binary = read_rom("files/roms/3-corax+.ch8").unwrap();

    let mut chip8 = chip8::Chip8::start(binary);

    let mut window = Window::new(
        "Chip-8 Emulator _ Use Esc to exit",
        SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
        SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    if debug_mode {
        run_debug_mode(instruction_count, &mut buffer, &mut chip8, &mut window);
    } else {
        run_normal_mode(&mut buffer, &mut chip8, &mut window);
    }
}

fn run_normal_mode(buffer: &mut Vec<u32>, chip8: &mut chip8::Chip8, window: &mut Window) {
    // Normal mode loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.tick();

        draw_screen_if_needed(buffer, chip8);

        update_window_with_buffer(buffer, window);
    }
}

fn run_debug_mode(instruction_count: usize, buffer: &mut Vec<u32>, chip8: &mut chip8::Chip8, window: &mut Window) {
    chip8.debug_mode = true; 

    // Debug mode loop
    let mut space_pressed = false;

    println!("CHIP-8 Debug Mode");
    println!("Controls:");
    println!("  SPACE - Execute one instruction");
    println!("  ESC   - Quit");
    println!();

    // Execute initial instructions if instruction_count > 0
    for _ in 0..instruction_count {
        chip8.tick();
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let space_down = window.is_key_down(Key::Space);
        if space_down && !space_pressed {
            chip8.tick();
        }
        space_pressed = space_down;

        draw_screen_if_needed(buffer, chip8);

        update_window_with_buffer(buffer, window);
    }
}

fn draw_screen_if_needed(buffer: &mut Vec<u32>, chip8: &chip8::Chip8) {
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

fn update_window_with_buffer(buffer: &mut Vec<u32>, window: &mut Window) {
    window
        .update_with_buffer(
            &buffer,
            SCREEN_WIDTH * SCREEN_SCALE_FACTOR,
            SCREEN_HEIGHT * SCREEN_SCALE_FACTOR,
        )
        .unwrap();
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
