use crate::chip8::Chip8;

pub struct Chip8Util;

impl Chip8Util {
    pub fn extract_nibbles(op_code: u16) -> (u16, u16, u16, u16) {
        let digit1: u16 = (op_code & 0xf000) >> 12;
        let digit2: u16 = (op_code & 0x0f00) >> 8;
        let digit3: u16 = (op_code & 0x00f0) >> 4;
        let digit4: u16 = op_code & 0x000f;
        (digit1, digit2, digit3, digit4)
    }

    pub fn print_instruction(chip8: &mut Chip8, op_code: u16) {
        let (digit1, digit2, digit3, digit4) = Chip8Util::extract_nibbles(op_code);

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
            chip8.instructions_executed, chip8.pc, op_code, instruction_desc
        );
        println!("-----------------------------------------------------");
        chip8.instructions_executed += 1;
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
    pub fn extract_digits(mut n: u8) -> Vec<u8> {
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
