use std::{
    fs::File,
    io::{self, BufRead, Read},
};

use minifb::{Key, Window, WindowOptions};

use crate::constants::{CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH};

mod chip8;
mod constants;

fn main() {
    let mut buffer: Vec<u32> = vec![0; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT];
    let chip8 = chip8::Chip8::new();
    let binary = read_rom("src/roms/IBM_Logo.ch8").unwrap();
    for byte in &binary {
        print!("{:02x} ", byte);
    }
    println!();

    for chunk in binary.chunks(2) {
        print!("{:02x?}", chunk);
    }
    println!();
    let instructions: Vec<u16> = binary
        .chunks(2)
        .map(|chunk| -> u16 {
            match chunk {
                [hi, lo] => u16::from_be_bytes([*hi, *lo]), // Big-endian (CHIP-8 standard)
                [single] => u16::from_be_bytes([0, *single]), // Handle odd length
                _ => 0,
            }
        })
        .collect();

    let mut window = Window::new(
        "Test - ESC to exit",
        CHIP8_SCREEN_WIDTH,
        CHIP8_SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    //FETCH
    //DECODE
    //EXECUTE

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     for i in buffer.iter_mut() {
    //         *i = 0xFFFFFF; // write something more funny here!
    //     }

    //     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    //     window
    //         .update_with_buffer(&buffer, CHIP8_SCREEN_WIDTH, CHIP8_SCREEN_HEIGHT)
    //         .unwrap();
    // }
    
    println!("Program is running...");
    println!("Press Enter to exit...");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
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
