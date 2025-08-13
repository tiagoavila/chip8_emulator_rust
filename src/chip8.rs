use crate::constants::{
    CHIP8_MEMORY_SIZE, CHIP8_REGISTER_COUNT, SCREEN_HEIGHT, SCREEN_WIDTH, START_RAM_ADDRESS,
};

pub struct Chip8 {
    pub memory: [u8; CHIP8_MEMORY_SIZE],
    pub pc: u16, // Program Counter
    pub i_register: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v_registers: [u8; CHIP8_REGISTER_COUNT],
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
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

        let op_digits = Chip8::extract_nibbles(op_code);
        let (digit1, digit2, digit3, digit4) = op_digits;

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
            (0xd, _, _, _) => self.draw_sprite_to_screen(op_digits),

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

    fn clear_screen() -> [bool; SCREEN_WIDTH * SCREEN_HEIGHT] {
        [false; SCREEN_WIDTH * SCREEN_HEIGHT]
    }

    fn set_v_register(&mut self, op_code: u16) {
        let v_register_index = ((op_code & 0x0f00) >> 8) as usize;
        let nn = (op_code & 0x00ff) as u8;
        self.v_registers[v_register_index] = nn;
    }

    // DXYN - Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in I
    // Set VF to 1 if any set pixels are changed to unset, and 0 otherwise
    fn draw_sprite_to_screen(&mut self, digits: (u16, u16, u16, u16)) {
        let x_register = digits.1 as usize;
        let y_register = digits.2 as usize;
        let bytes_to_read_count = digits.3 as usize;

        let x_coord = self.v_registers[x_register] as usize;
        let y_coord = self.v_registers[y_register] as usize;
        let i_register_value = self.i_register as usize;
        let mut pixel_change_to_unset = false;

        for row in i_register_value..(i_register_value + bytes_to_read_count) {
            let sprite_byte = &self.memory[row];
            for col in 0..8 {
                // Extract the current bit from the sprite byte
                // We start from the most significant bit (bit 7) and work down
                // Right shift by (7 - col) to move desired bit to position 0
                // Then AND with 1 to isolate just that bit (0 or 1)
                let sprite_pixel = (sprite_byte >> (7 - col)) & 1;

                // If the sprite is positioned so part of it is outside the coordinates of the display,
                // it wraps around to the opposite side of the screen.
                let screen_x = (col + x_coord) as usize % SCREEN_WIDTH;
                let screen_y = (row + y_coord) as usize % SCREEN_HEIGHT;

                // Get the pixel position in the screen as an array (formula: screen_y * SCREEN_WIDTH + screen_x)
                // This is the index in the screen array where we will set the pixel, example: considering a 64x32 screen:
                // For screen_y = 0 and screen_x = 0, index = 0
                // For screen_y = 0 and screen_x = 63, index = 63
                // For screen_y = 1 and screen_x = 0, index = 64
                let screen_index = screen_y * SCREEN_WIDTH + screen_x;
                let old_pixel = self.screen[screen_index];

                // XOR the current value in the screen with true
                let new_pixel = self.screen[screen_index] ^ (sprite_pixel == 1);
                self.screen[screen_index] = new_pixel;

                // If the XOR causes any pixels to be erased (set from true to false), VF is set to 1, otherwise it is set to 0
                if old_pixel == true && new_pixel == false {
                    pixel_change_to_unset = true;
                }
            }
        }

        if pixel_change_to_unset {
            self.v_registers[0xF] = 1;
        } else {
            self.v_registers[0xF] = 0;
        }
    }
}
