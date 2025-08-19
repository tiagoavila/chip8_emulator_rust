pub const CHIP8_RAM_MEMORY_SIZE: usize = 4096;
pub const CHIP8_STACK_MEMORY_SIZE: usize = 16;
pub const CHIP8_REGISTER_COUNT: usize = 16;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_SCALE_FACTOR: usize = 10;
pub const START_RAM_ADDRESS: u16 = 0x200;
pub const CLEANED_SCREEN: [bool; SCREEN_WIDTH * SCREEN_HEIGHT] = [false; SCREEN_WIDTH * SCREEN_HEIGHT];