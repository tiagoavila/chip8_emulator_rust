use crate::constants::{
    CHIP8_MEMORY_SIZE, CHIP8_REGISTER_COUNT, CHIP8_SCREEN_HEIGHT, CHIP8_SCREEN_WIDTH,
    START_RAM_ADDRESS,
};

pub struct Chip8 {
    pub memory: [u8; CHIP8_MEMORY_SIZE],
    pub pc: u16, // Program Counter
    pub i_register: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v_registers: [u8; CHIP8_REGISTER_COUNT],
    pub screen: [u8; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; CHIP8_MEMORY_SIZE],
            pc: START_RAM_ADDRESS, // Programs start at memory location 0x200
            i_register: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v_registers: [0; CHIP8_REGISTER_COUNT],
            screen: Chip8::clear_screen(),
        }
    }

    pub fn tick(&mut self) {
        //FETCH
        let op_code = self.fetch();

        //DECODE
        //EXECUTE
        self.decode_execute(op_code);
        
        self.pc += 2; // Move to the next instruction
    }

    pub fn fetch(&mut self) -> u16 {
        let high_byte = self.memory.get(self.pc as usize);
        let low_byte = self.memory.get((self.pc + 1) as usize);
        if high_byte.is_some() && low_byte.is_some() {
            let high_byte = high_byte.unwrap();
            let low_byte = low_byte.unwrap();

            return u16::from_be_bytes([*high_byte, *low_byte]);
        }

        0
    }

    pub fn decode_execute(&mut self, op_code: u16) {
        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction

        let digits = Chip8::extract_nibbles(op_code);
        let (digit1, digit2, digit3, digit4) = digits;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => return,

            // 00E0 - CLS - Clear screen
            (0, 0, 0xe, 0) => self.screen = Self::clear_screen(),

            // 6xnn - vx register = nn
            (0x6, _, _, _) => self.set_v_register(op_code),

            // Annn - set value of i_register to nnn
            (0xa, _, _, _) => self.i_register = op_code & 0x0fff,

            // Dxyn - Draw sprite to screen
            (0xd, _, _, _) => self.draw_sprite_to_screen(op_code, digits),

            _ => return,
        }
    }

    fn extract_nibbles(op_code: u16) -> (u16, u16, u16, u16) {
        let digit1: u16 = (op_code & 0xf000) >> 12;
        let digit2: u16 = (op_code & 0x0f00) >> 8;
        let digit3: u16 = (op_code & 0x00f0) >> 4;
        let digit4: u16 = op_code & 0x000f;
        (digit1, digit2, digit3, digit4)
    }

    fn clear_screen() -> [u8; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT] {
        [0; CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT]
    }

    fn set_v_register(&mut self, op_code: u16) {
        let v_register_index = ((op_code & 0x0f00) >> 8) as usize;
        let nn = (op_code & 0x00ff) as u8;
        self.v_registers[v_register_index] = nn;
    }

    // DXYN - Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    // Set VF to 01 if any set pixels are changed to unset, and 00 otherwise
    fn draw_sprite_to_screen(&mut self, op_code: u16, digits: (u16, u16, u16, u16)) {
        let x = digits.1 as usize;
        let y = digits.2 as usize;
        let n = digits.3 as usize;

        let x_value = self.v_registers[x];
        let y_value = self.v_registers[y];

        let i_register_value = self.i_register as usize;
        for row in i_register_value..(i_register_value + n) 
        {
            let sprite_byte = &self.memory[row];
            for bit in 0..8 {
            }
        }
    }
}
