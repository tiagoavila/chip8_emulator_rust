use rand::Rng;

use crate::constants::{
    CHIP8_RAM_MEMORY_SIZE, CHIP8_REGISTER_COUNT, CHIP8_STACK_MEMORY_SIZE, CLEANED_SCREEN,
    SCREEN_HEIGHT, SCREEN_WIDTH, START_RAM_ADDRESS,
};

pub struct Chip8 {
    pub ram: [u8; CHIP8_RAM_MEMORY_SIZE],
    pub pc: u16,         // Program Counter
    pub i_register: u16, // This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used
    pub stack: [u16; CHIP8_STACK_MEMORY_SIZE],
    pub stack_pointer: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub v_registers: [u8; CHIP8_REGISTER_COUNT],
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub needs_redraw: bool,
    pub debug_mode: bool, // Flag to indicate if the emulator is in debug mode
    pub instructions_executed: usize, // Count of instructions executed
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: [0; CHIP8_RAM_MEMORY_SIZE],
            pc: START_RAM_ADDRESS, // Programs start at memory location 0x200
            i_register: 0,
            stack: [0; CHIP8_STACK_MEMORY_SIZE],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            v_registers: [0; CHIP8_REGISTER_COUNT],
            screen: CLEANED_SCREEN,
            needs_redraw: false,
            debug_mode: false,
            instructions_executed: 0,
        }
    }

    pub fn start(rom_binary: Vec<u8>) -> Self {
        let mut chip8 = Self::new();
        chip8.load_rom(rom_binary);
        return chip8;
    }

    pub fn tick(&mut self) {
        self.needs_redraw = false;

        //FETCH
        let op_code = self.fetch();

        if self.debug_mode {
            self.print_instruction(op_code);
        }

        //DECODE
        //EXECUTE
        self.decode_execute(op_code);
    }

    /// Fetch the next opcode (2 bytes) from memory at the current program counter
    pub fn fetch(&mut self) -> u16 {
        let high_byte = self.ram.get(self.pc as usize);
        let low_byte = self.ram.get((self.pc + 1) as usize);
        if high_byte.is_some() && low_byte.is_some() {
            let high_byte = high_byte.unwrap();
            let low_byte = low_byte.unwrap();

            self.pc += 2; // Move to the next instruction

            return u16::from_be_bytes([*high_byte, *low_byte]);
        }

        0
    }

    /// Decode and execute the given opcode
    pub fn decode_execute(&mut self, op_code: u16) {
        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction

        let op_digits = Chip8::extract_nibbles(op_code);
        let (digit1, digit2, digit3, digit4) = op_digits;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            (0, 0, 0xe, 0) => self.clear_screen(),
            (0, 0, 0xe, 0xe) => self.return_from_subroutine(),
            (0, _, _, _) => self.sys_addr(op_code),
            (0x1, _, _, _) => self.jump(op_code),
            (0x2, _, _, _) => self.call_subroutine(op_code),
            (0x3, _, _, _) => self.skip_if_equal(digit2, digit3, digit4),
            (0x4, _, _, _) => self.skip_if_vx_not_eq_kk(digit2, digit3, digit4),
            (0x5, _, _, 0) => self.skip_if_vx_eq_vy(digit2, digit3),
            (0x6, _, _, _) => self.set_v_register(op_code),
            (0x7, _, _, _) => self.add_value_to_v_register(op_code),
            (0x8, _, _, 0) => self.store_vy_in_vx(digit2, digit3),
            (0x8, _, _, 1) => self.set_vx_with_vx_or_vy(digit2, digit3),
            (0x8, _, _, 2) => self.set_vx_with_vx_and_vy(digit2, digit3),
            (0x8, _, _, 3) => self.set_vx_with_vx_xor_vy(digit2, digit3),
            (0x8, _, _, 4) => self.add_vx_with_vy(digit2, digit3),
            (0x8, _, _, 5) => self.subtract_vy_from_vx(digit2, digit3),
            (0x8, _, _, 6) => self.shr_vx(digit2),
            (0x8, _, _, 7) => self.subtract_vx_from_vy(digit2, digit3),
            (0x8, _, _, 0xe) => self.shl_vx(digit2),
            (0x9, _, _, 0) => self.skip_if_vx_ne_vy(digit2, digit3),
            (0xa, _, _, _) => self.set_i_register(op_code),
            (0xb, _, _, _) => self.jump_v0_addr(op_code),
            (0xc, _, _, _) => self.rnd_vx_byte(op_code),
            (0xd, _, _, _) => self.draw_sprite_to_screen(op_digits),
            (0xe, _, 9, 0xe) => self.skp_vx(digit2),
            (0xe, _, 0xa, 1) => self.sknp_vx(digit2),
            (0xf, _, 0, 7) => self.ld_vx_dt(digit2),
            (0xf, _, 0, 0xa) => self.ld_vx_k(digit2),
            (0xf, _, 1, 5) => self.ld_dt_vx(digit2),
            (0xf, _, 1, 8) => self.ld_st_vx(digit2),
            (0xf, _, 1, 0xe) => self.add_vx_to_i(digit2),
            (0xf, _, 2, 9) => self.ld_f_vx(digit2),
            (0xf, _, 3, 3) => self.store_bcd_of_vx_in_memory(digit2),
            (0xf, _, 5, 5) => self.fill_memory_with_v0_to_vx(digit2),
            (0xf, _, 6, 5) => self.fill_v0_to_vx_starting_at_i(digit2),
            _ => return,
        }
    }

    pub fn enable_debug_mode(&mut self, instructions_executed: usize) {
        self.debug_mode = true;
        self.instructions_executed = instructions_executed;
    }

    fn load_rom(&mut self, rom_binary: Vec<u8>) {
        let start_ram_address = START_RAM_ADDRESS as usize;
        self.ram[start_ram_address..(start_ram_address + rom_binary.len())]
            .copy_from_slice(&rom_binary);
    }

    fn set_i_register(&mut self, op_code: u16) {
        self.i_register = op_code & 0x0fff
    }

    fn extract_nibbles(op_code: u16) -> (u16, u16, u16, u16) {
        let digit1: u16 = (op_code & 0xf000) >> 12;
        let digit2: u16 = (op_code & 0x0f00) >> 8;
        let digit3: u16 = (op_code & 0x00f0) >> 4;
        let digit4: u16 = op_code & 0x000f;
        (digit1, digit2, digit3, digit4)
    }

    fn clear_screen(&mut self) {
        self.screen = CLEANED_SCREEN;
        self.needs_redraw = true;
    }

    fn set_v_register(&mut self, op_code: u16) {
        let v_register_index = ((op_code & 0x0f00) >> 8) as usize;
        let nn = (op_code & 0x00ff) as u8;
        self.v_registers[v_register_index] = nn;
    }

    /// 7XNN - Add the value NN to register VX.
    /// Be aware that once the supplied number is added, if the value of the register exceeds decimal 255
    /// (the highest possible value that can be stored by an eight bit register),
    /// the register will wraparound to a corresponding value that can be stored by an eight bit register.
    /// In other words, the register will always be reduced modulo decimal 256.
    fn add_value_to_v_register(&mut self, op_code: u16) {
        let v_register_index = ((op_code & 0x0f00) >> 8) as usize;
        let nn = (op_code & 0x00ff) as u8;
        self.v_registers[v_register_index] = self.v_registers[v_register_index].wrapping_add(nn);
    }

    // DXYN - Draw a sprite at position VX, VY with N bytes of sprite data starting at the address stored in the I register
    // Set VF to 1 if any set pixels are changed to unset, and 0 otherwise
    // Chip-8’s sprites are always 8 pixels wide, but can be a variable number of pixels tall, from 1 to 16.
    // This is specified in the final digit of the opcode.
    fn draw_sprite_to_screen(&mut self, digits: (u16, u16, u16, u16)) {
        let vx_register = digits.1 as usize;
        let vy_register = digits.2 as usize;
        let sprite_height = digits.3 as usize;

        let x_coord = self.v_registers[vx_register] as usize;
        let y_coord = self.v_registers[vy_register] as usize;
        let i_register_value = self.i_register as usize;
        let mut pixel_change_to_unset = false;

        // Loop through each row of the sprite (height determines number of rows)
        for row in 0..sprite_height {
            // Read one byte from memory starting at I register + current row offset
            // Each byte represents 8 pixels (one row of the sprite)
            let sprite_byte = &self.ram[i_register_value + row];
            // println!("Sprite byte (row {:2}): {:08b}", row, sprite_byte);

            // Process each of the 8 bits in this byte (8 pixels per row)
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
        self.needs_redraw = true;
    }

    fn jump(&mut self, op_code: u16) {
        self.pc = op_code & 0x0fff;
    }

    fn print_instruction(&mut self, op_code: u16) {
        let (digit1, digit2, digit3, digit4) = Chip8::extract_nibbles(op_code);

        let instruction_desc = match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => {
                "0nnn - SYS addr\nJump to a machine code routine at nnn. Ignored by modern interpreters."
            }
            (0, 0, 0xE, 0) => "00E0 - CLS\nClear the display.",
            (0, 0, 0xE, 0xE) => {
                "00EE - RET\nReturn from a subroutine. The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer."
            }
            (0x1, _, _, _) => {
                "1nnn - JP addr\nJump to location nnn. The interpreter sets the program counter to nnn."
            }
            (0x2, _, _, _) => {
                "2nnn - CALL addr\nCall subroutine at nnn. The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn."
            }
            (0x3, _, _, _) => {
                "3xkk - SE Vx, byte\nSkip next instruction if Vx = kk. The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2."
            }
            (0x4, _, _, _) => {
                "4xkk - SNE Vx, byte\nSkip next instruction if Vx != kk. The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2."
            }
            (0x5, _, _, 0) => {
                "5xy0 - SE Vx, Vy\nSkip next instruction if Vx = Vy. The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2."
            }
            (0x6, _, _, _) => {
                "6xkk - LD Vx, byte\nSet Vx = kk. The interpreter puts the value kk into register Vx."
            }
            (0x7, _, _, _) => {
                "7xkk - ADD Vx, byte\nSet Vx = Vx + kk. Adds the value kk to the value of register Vx, then stores the result in Vx."
            }
            (0x8, _, _, 0) => {
                "8xy0 - LD Vx, Vy\nSet Vx = Vy. Stores the value of register Vy in register Vx."
            }
            (0x8, _, _, 1) => {
                "8xy1 - OR Vx, Vy\nSet Vx = Vx OR Vy. Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx."
            }
            (0x8, _, _, 2) => {
                "8xy2 - AND Vx, Vy\nSet Vx = Vx AND Vy. Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx."
            }
            (0x8, _, _, 3) => {
                "8xy3 - XOR Vx, Vy\nSet Vx = Vx XOR Vy. Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx."
            }
            (0x8, _, _, 4) => {
                "8xy4 - ADD Vx, Vy\nSet Vx = Vx + Vy, set VF = carry. The values of Vx and Vy are added together. If the result is greater than 8 bits, VF is set to 1, otherwise 0."
            }
            (0x8, _, _, 5) => {
                "8xy5 - SUB Vx, Vy\nSet Vx = Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx."
            }
            (0x8, _, _, 6) => {
                "8xy6 - SHR Vx\nSet Vx = Vx SHR 1. If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2."
            }
            (0x8, _, _, 7) => {
                "8xy7 - SUBN Vx, Vy\nSet Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx."
            }
            (0x8, _, _, 0xE) => {
                "8xyE - SHL Vx\nSet Vx = Vx SHL 1. If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2."
            }
            (0x9, _, _, 0) => {
                "9xy0 - SNE Vx, Vy\nSkip next instruction if Vx != Vy. The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2."
            }
            (0xA, _, _, _) => {
                "Annn - LD I, addr\nSet I = nnn. The value of register I is set to nnn."
            }
            (0xB, _, _, _) => {
                "Bnnn - JP V0, addr\nJump to location nnn + V0. The program counter is set to nnn plus the value of V0."
            }
            (0xC, _, _, _) => {
                "Cxkk - RND Vx, byte\nSet Vx = random byte AND kk. The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk."
            }
            (0xD, _, _, _) => {
                "Dxyn - DRW Vx, Vy, nibble\nDisplay n-byte sprite starting at memory location I at (Vx, Vy). Sprites are XORed onto the existing screen."
            }
            (0xE, _, 9, 0xE) => {
                "Ex9E - SKP Vx\nSkip next instruction if key with the value of Vx is pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2."
            }
            (0xE, _, 0xA, 1) => {
                "ExA1 - SKNP Vx\nSkip next instruction if key with the value of Vx is not pressed. Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2."
            }
            (0xF, _, 0, 7) => {
                "Fx07 - LD Vx, DT\nSet Vx = delay timer value. The value of DT is placed into Vx."
            }
            (0xF, _, 0, 0xA) => {
                "Fx0A - LD Vx, K\nWait for a key press, store the value of the key in Vx. All execution stops until a key is pressed, then the value of that key is stored in Vx."
            }
            (0xF, _, 1, 5) => {
                "Fx15 - LD DT, Vx\nSet delay timer = Vx. DT is set equal to the value of Vx."
            }
            (0xF, _, 1, 8) => {
                "Fx18 - LD ST, Vx\nSet sound timer = Vx. ST is set equal to the value of Vx."
            }
            (0xF, _, 1, 0xE) => {
                "Fx1E - ADD I, Vx\nSet I = I + Vx. The values of I and Vx are added, and the results are stored in I."
            }
            (0xF, _, 2, 9) => {
                "Fx29 - LD F, Vx\nSet I = location of sprite for digit Vx. The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx."
            }
            (0xF, _, 3, 3) => {
                "Fx33 - LD B, Vx\nStore BCD representation of Vx in memory locations I, I+1, and I+2. The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2."
            }
            (0xF, _, 5, 5) => {
                "Fx55 - LD [I], Vx\nStore registers V0 through Vx in memory starting at location I. The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I."
            }
            (0xF, _, 6, 5) => {
                "Fx65 - LD Vx, [I]\nRead registers V0 through Vx from memory starting at location I. The interpreter reads values from memory starting at location I into registers V0 through Vx."
            }
            _ => "Unknown instruction",
        };

        println!(
            "Instruction Count: {} - PC: {:04x} - OpCode: {:04x}\n{}",
            self.instructions_executed, self.pc, op_code, instruction_desc
        );
        println!("-----------------------------------------------------");
        self.instructions_executed += 1;
    }

    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the Vx register to compare.
    /// * `k1` - The high nibble of the byte to compare.
    /// * `k2` - The low nibble of the byte to compare.
    ///
    /// # Behavior
    ///
    /// Combines `k1` and `k2` into an 8-bit value (kk) and compares it with the value in the Vx register.
    /// If they are equal, the program counter (PC) is incremented by 2 to skip the next instruction.
    fn skip_if_equal(&mut self, x: u16, k1: u16, k2: u16) {
        let v_register_value = self.v_registers[x as usize];
        let kk: u8 = ((k1 << 4) | k2) as u8;
        if v_register_value == kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx != kk.
    /// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    ///
    /// # Arguments
    ///
    /// * `x` - The index of the Vx register to compare.
    /// * `k1` - The high nibble of the byte to compare.
    /// * `k2` - The low nibble of the byte to compare.
    ///
    /// # Behavior
    ///
    /// Combines `k1` and `k2` into an 8-bit value (kk) and compares it with the value in the Vx register.
    /// If they are not equal, the program counter (PC) is incremented by 2 to skip the next instruction.
    fn skip_if_vx_not_eq_kk(&mut self, x: u16, k1: u16, k2: u16) {
        let v_register_value = self.v_registers[x as usize];
        let kk: u8 = ((k1 << 4) | k2) as u8;
        if v_register_value != kk {
            self.pc += 2;
        }
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// The interpreter puts the current PC on the top of the stack and increments the stack pointer. The PC is then set to nnn.
    fn call_subroutine(&mut self, op_code: u16) {
        self.stack[self.stack_pointer] = self.pc;
        self.stack_pointer += 1;
        self.pc = op_code & 0x0fff;
    }

    /// 00EE - Return from a subroutine.
    /// The interpreter subtracts 1 from the stack pointer and sets the program counter to the address at the top of the stack.
    fn return_from_subroutine(&mut self) {
        self.stack_pointer -= 1;
        self.pc = self.stack[self.stack_pointer];
    }

    /// 8xy0 - LD Vx, Vy.
    /// Set Vx = Vy.
    /// Stores the value of register Vy in register Vx.
    /// # Arguments
    ///
    /// * `x` - The index of the Vx register to store the value in.
    /// * `y` - The index of the Vy register to read the value from.
    fn store_vy_in_vx(&mut self, x: u16, y: u16) {
        self.v_registers[x as usize] = self.v_registers[y as usize];
    }

    /// 8xy1 - OR Vx, Vy.
    /// Set Vx = Vx OR Vy.
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    /// The | operator compares each bit of the left and right operands.
    fn set_vx_with_vx_or_vy(&mut self, x: u16, y: u16) {
        self.v_registers[x as usize] |= self.v_registers[y as usize];
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
    /// A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
    /// then the same bit in the result is also 1. Otherwise, it is 0.
    fn set_vx_with_vx_and_vy(&mut self, x: u16, y: u16) {
        self.v_registers[x as usize] &= self.v_registers[y as usize];
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
    /// An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same,
    /// then the corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn set_vx_with_vx_xor_vy(&mut self, x: u16, y: u16) {
        self.v_registers[x as usize] ^= self.v_registers[y as usize];
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
    /// VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
    fn add_vx_with_vy(&mut self, x: u16, y: u16) {
        let vx = self.v_registers[x as usize];
        let vy = self.v_registers[y as usize];

        let (result, overflowed) = vx.overflowing_add(vy);

        self.v_registers[x as usize] = result;

        if overflowed {
            self.v_registers[0xF] = 1;
        } else {
            self.v_registers[0xF] = 0;
        }
    }

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    fn subtract_vy_from_vx(&mut self, x: u16, y: u16) {
        let vx = self.v_registers[x as usize];
        let vy = self.v_registers[y as usize];

        if vx > vy {
            self.v_registers[0xF] = 1;
        } else {
            self.v_registers[0xF] = 0;
        }

        self.v_registers[x as usize] = vx.wrapping_sub(vy);
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    fn shr_vx(&mut self, x: u16) {
        self.v_registers[0xF] = self.v_registers[x as usize] & 0x01;

        self.v_registers[x as usize] >>= 1;
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    fn subtract_vx_from_vy(&mut self, x: u16, y: u16) {
        let vx = self.v_registers[x as usize];
        let vy = self.v_registers[y as usize];

        if vy > vx {
            self.v_registers[0xF] = 1;
        } else {
            self.v_registers[0xF] = 0;
        }

        self.v_registers[x as usize] = vy.wrapping_sub(vx);
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    fn shl_vx(&mut self, x: u16) {
        self.v_registers[0xF] = (self.v_registers[x as usize] >> 7) & 0x01;

        self.v_registers[x as usize] <<= 1;
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn add_vx_to_i(&mut self, x: u16) {
        self.i_register = self
            .i_register
            .wrapping_add(self.v_registers[x as usize] as u16);
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
    /// the tens digit at location I+1, and the ones digit at location I+2.
    fn store_bcd_of_vx_in_memory(&mut self, x: u16) {
        let vx = self.v_registers[x as usize];
        let mut bcd_vx: Vec<u8> = Chip8::extract_digits(vx);
        if bcd_vx.len() < 3 {
            let diff = 3 - bcd_vx.len();
            for _ in 0..diff {
                bcd_vx.insert(0, 0);
            }
        }

        for (index, bcd) in bcd_vx.iter().enumerate() {
            self.ram[index + self.i_register as usize] = *bcd;
        }
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    ///
    /// More detailed info:
    /// Store the values of registers V0 to VX inclusive in memory starting at address I.
    /// I is set to I + X + 1 after operation
    fn fill_memory_with_v0_to_vx(&mut self, x: u16) {
        for v_register_index in 0..=x as usize {
            self.ram[self.i_register as usize + v_register_index] =
                self.v_registers[v_register_index];
        }

        self.i_register += x + 1;
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    ///
    /// More detailed info:
    /// Fill registers V0 to VX inclusive with the values stored in memory starting at address I.
    /// I is set to I + X + 1 after operation
    fn fill_v0_to_vx_starting_at_i(&mut self, x: u16) {
        for v_register_index in 0..=x as usize {
            self.v_registers[v_register_index] =
                self.ram[self.i_register as usize + v_register_index];
        }

        self.i_register += x + 1;
    }

    /// 0nnn - SYS addr
    /// Jump to a machine code routine at nnn. Ignored by most modern interpreters.
    fn sys_addr(&mut self, _op_code: u16) {
        return;
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    fn skip_if_vx_eq_vy(&mut self, x: u16, y: u16) {
        let vx = self.v_registers[x as usize];
        let vy = self.v_registers[y as usize];
        if vx == vy {
            self.pc += 2;
        }
    }

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.    
    fn skip_if_vx_ne_vy(&mut self, x: u16, y: u16) {
        let vx = self.v_registers[x as usize];
        let vy = self.v_registers[y as usize];
        if vx != vy {
            self.pc += 2;
        }
    }

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// The program counter is set to nnn plus the value of V0.
    fn jump_v0_addr(&mut self, op_code: u16) {
        self.pc = (op_code & 0x0fff) + self.v_registers[0] as u16;
    }

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. 
    /// The results are stored in Vx. See instruction 8xy2 for more information on AND.
    fn rnd_vx_byte(&mut self, op_code: u16) {
        let mut rng = rand::thread_rng();
        let random_byte: u8 = rng.gen_range(0..=255);

        let v_register_index = ((op_code & 0x0f00) >> 8) as usize;
        let kk = (op_code & 0x00ff) as u8;

        self.v_registers[v_register_index] = random_byte & kk;
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    fn skp_vx(&mut self, _x: u16) {
        unimplemented!("SKP Vx (Ex9E) is not implemented");
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    fn sknp_vx(&mut self, _x: u16) {
        unimplemented!("SKNP Vx (ExA1) is not implemented");
    }

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// The value of DT is placed into Vx.
    fn ld_vx_dt(&mut self, x: u16) {
        self.v_registers[x as usize] = self.delay_timer;
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    fn ld_vx_k(&mut self, _x: u16) {
        unimplemented!("LD Vx, K (Fx0A) is not implemented");
    }

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// DT is set equal to the value of Vx.
    fn ld_dt_vx(&mut self, x: u16) {
        self.delay_timer = self.v_registers[x as usize];
    }

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// ST is set equal to the value of Vx.
    fn ld_st_vx(&mut self, x: u16) {
        self.sound_timer = self.v_registers[x as usize];
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. 
    fn ld_f_vx(&mut self, x: u16) {
        // self.i_register = 
    }

    /// Extracts the decimal digits of a number in order (most to least significant).
    ///
    /// Given an input `n`, returns a vector of its digits, starting from the most significant digit.
    /// For example, `extract_digits(153)` returns `[1, 5, 3]`.
    /// If `n` is 0, returns `[0]`.
    ///
    /// # Example
    /// ```
    /// let digits = extract_digits(153);
    /// assert_eq!(digits, vec![1, 5, 3]);
    /// ```
    fn extract_digits(mut n: u8) -> Vec<u8> {
        if n == 0 {
            return vec![0];
        }

        let mut digits: Vec<u8> = Vec::new();

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.reverse();

        return digits;
    }
}
