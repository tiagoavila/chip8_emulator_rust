use crate::constants::{CHIP8_MEMORY_SIZE, CHIP8_REGISTER_COUNT};

pub struct Chip8 {
   pub memory: [u8; CHIP8_MEMORY_SIZE],
   pub pc: u16, // Program Counter
   pub register_i: u16,
   pub stack: Vec<u16>,
   pub delay_timer: u8,
   pub sound_timer: u8,
   pub registers: [u8; CHIP8_REGISTER_COUNT]
}

impl Chip8 {
   pub fn new() -> Self {
      Self {
         memory: [0; CHIP8_MEMORY_SIZE],
         pc: 0x200, // Programs start at memory location 0x200
         register_i: 0,
         stack: Vec::new(),
         delay_timer: 0,
         sound_timer: 0,
         registers: [0; CHIP8_REGISTER_COUNT],
      }
   }
}