pub struct Chip8 {
   pub memory: [u8; 4096],
   pub pc: u16, // Program Counter
   pub register_i: u16,
   pub stack: Vec<u16>,
   pub delay_timer: u8,
   pub sound_timer: u8,
   pub registers: [u8; 16]
}