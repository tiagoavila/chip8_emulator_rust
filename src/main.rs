use std::{
    fmt::format,
    fs::File,
    io::{self, BufRead, Read},
};

use minifb::{Key, Window, WindowOptions};

use crate::constants::{CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH};

mod chip8;
mod constants;

fn main() {
    let mut buffer: Vec<u32> = vec![0; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT];
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

    // for byte in &binary {
    //     print!("{:02x} ", byte);
    // }
    // println!();

    // for chunk in binary.chunks(2) {
    //     print!("{:02x?}", chunk);
    // }
    // println!();
    // let instructions: Vec<u16> = binary
    //     .chunks(2)
    //     .map(|chunk| -> u16 {
    //         match chunk {
    //             [hi, lo] => u16::from_be_bytes([*hi, *lo]), // Big-endian (CHIP-8 standard)
    //             [single] => u16::from_be_bytes([0, *single]), // Handle odd length
    //             _ => 0,
    //         }
    //     })
    //     .collect();
    let first_instruction = format!("{:04x}", chip8.fetch());
    chip8.pc += 2;
    println!("{:04x}", chip8.fetch());
    println!("{:04b}", chip8.fetch());
    let second_instruction = chip8.fetch();
    println!("{:04b}", second_instruction);
    println!("{:04b}", (second_instruction & 0xF000) >> 12);
    println!("{:04b}", (second_instruction & 0x0F00) >> 8);
    println!("{:04b}", (second_instruction & 0x00F0) >> 4);
    println!("{:04b}", (second_instruction & 0x000F));

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0xFFFFFF; // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, CHIP8_SCREEN_WIDTH, CHIP8_SCREEN_HEIGHT)
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
