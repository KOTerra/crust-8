use crate::input::Input;
use rand::Rng;
use std::fs::File;
use std::io::{BufReader, Read};

const FONTSET_ADDRESS_START: usize = 0x50;
const FONTSET_ADDRESS_END: usize = 0x80;
const PROGRAM_START: usize = 0x200;
const MEMORY_SIZE: usize = 4096;
const PIXELS_WIDE: usize = 64;
const PIXELS_HIGH: usize = 32;

pub(crate) struct Chip8Cpu {
    v_registers: [u8; 16], //v[0] - v[F], F is flags register
    i_register: u16,
    stack: Vec<u16>, //size 16
    stack_ptr: u16,
    program_counter: u16,
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,

    ram: [u8; MEMORY_SIZE],

    pub(crate) display: [bool; PIXELS_WIDE * PIXELS_HIGH],
    pub(crate) draw_flag: bool,

    keys: [bool; 16],

    opcode: u16,
}

impl Chip8Cpu {
    pub(crate) fn new() -> Self {
        let mut returned: Chip8Cpu = Self {
            v_registers: [0; 16],
            i_register: 0,
            stack: vec![0, 16],
            stack_ptr: 0,
            program_counter: PROGRAM_START as u16,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; MEMORY_SIZE],
            display: [false; PIXELS_WIDE * PIXELS_HIGH],
            draw_flag: false,
            keys: [false; 16],
            opcode: 0,
        };
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..80 {
            returned.ram[FONTSET_ADDRESS_START + i] = font[i];
        }

        returned
    }
    pub(crate) fn recieve_input(&mut self, input: &Input) {
        self.keys[1] = input.key_1; //1
        self.keys[2] = input.key_2; //2
        self.keys[3] = input.key_3; //3
        self.keys[12] = input.key_4; //C

        self.keys[4] = input.key_q; //4
        self.keys[5] = input.key_w; //5
        self.keys[6] = input.key_e; //6
        self.keys[13] = input.key_r; //D

        self.keys[7] = input.key_a; //7
        self.keys[8] = input.key_s; //8
        self.keys[9] = input.key_d; //9
        self.keys[14] = input.key_f; //E

        self.keys[10] = input.key_z; //A
        self.keys[0] = input.key_x; //0
        self.keys[11] = input.key_c; //B
        self.keys[15] = input.key_v; //F
    }

    pub(crate) fn open_rom(&mut self, input: &Input) {
        let mut f = BufReader::new(File::open(input.file_name.clone()).expect("open failed"));

        let mut buf = vec![0u8; MEMORY_SIZE - PROGRAM_START];

        loop {
            let bytes_read = f.read(&mut buf).expect("read failed");
            if bytes_read == 0 {
                break;
            }

            let mut start_index = PROGRAM_START; //512

            let end_index = start_index + bytes_read;
            if end_index <= self.ram.len() {
                self.ram[start_index..end_index].copy_from_slice(&buf[..bytes_read]);
            } else {
                eprintln!("Not enough space in RAM to copy ROM data!");
            }
            for byte in &buf[..bytes_read] {
                println!("Byte{}: {:#04x}", start_index, byte);
                start_index += 1;
            }
            println!("\n\n\n");
        }
    }

    pub(crate) fn reset(&mut self) {
        self.program_counter = PROGRAM_START as u16;
        self.stack = vec![0];
        self.stack_ptr = 0;
        self.opcode = 0;
        let _ = &self.ram
            [self.program_counter as usize..self.program_counter as usize + PROGRAM_START]
            .fill(0x00);
    }

    pub(crate) fn execute_cycle(&mut self) {
        self.fetch();
        if (self.program_counter as usize) < MEMORY_SIZE - 2 {
            self.program_counter += 2;
        }
        self.decode_execute();
    }
    fn fetch(&mut self) {
        self.opcode = (self.ram[self.program_counter as usize] as u16) << 8
            | (self.ram[self.program_counter as usize + 1] as u16);
    }
    fn decode_execute(&mut self) {
        let opcode = self.opcode;
       // println!("Opcode: {:04X}", opcode);
        let first_nibble = (opcode & 0xF000) >> 12;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;

        match first_nibble {
            0x0 => match opcode {
                0x00E0 => self.op_00e0(), 
                0x00EE => self.op_00ee(), 
                _ => {}                   
            },
            0x1 => self.op_1nnn(), // Jump to address
            0x2 => self.op_2nnn(), // Call subroutine
            0x3 => self.op_3xkk(), // Skip if VX == NN
            0x4 => self.op_4xkk(), // Skip if VX != NN
            0x5 => self.op_5xy0(), // Skip if VX == VY
            0x6 => self.op_6xkk(), // Set VX to NN
            0x7 => self.op_7xkk(), // Add NN to VX
            0x8 => match n {
                0x0 => self.op_8xy0(), // VX = VY
                0x1 => self.op_8xy1(), // VX = VX | VY
                0x2 => self.op_8xy2(), // VX = VX & VY
                0x3 => self.op_8xy3(), // VX = VX ^ VY
                0x4 => self.op_8xy4(), // VX += VY (carry flag)
                0x5 => self.op_8xy5(), // VX -= VY (borrow flag)
                0x6 => self.op_8xy6(), // Shift VX right
                0x7 => self.op_8xy7(), // VX = VY - VX
                0xE => self.op_8xye(), // Shift VX left
                _ => {}
            },
            0x9 => self.op_9xy0(), // Skip if VX != VY
            0xA => self.op_annn(), // Set I register
            0xB => self.op_bnnn(), // Jump to NNN + V0
            0xC => self.op_cxkk(), // Random number & NN
            0xD => self.op_dxyn(), // Draw sprite
            0xE => match nn {
                0x9E => self.op_ex9e(), // Skip if key in VX is pressed
                0xA1 => self.op_exa1(), // Skip if key in VX is NOT pressed
                _ => {}
            },
            0xF => match nn {
                0x07 => self.op_fx07(), // Get delay timer
                0x0A => self.op_fx0a(), // Wait for key press
                0x15 => self.op_fx15(), // Set delay timer
                0x18 => self.op_fx18(), // Set sound timer
                0x1E => self.op_fx1e(), // Add VX to I
                0x29 => self.op_fx29(), // Set I to font sprite
                0x33 => self.op_fx33(), // Store BCD of VX
                0x55 => self.op_fx55(), // Store registers in memory
                0x65 => self.op_fx65(), // Load registers from memory
                _ => {}
            },
            _ => {} // Unknown opcode
        }
    }

    //clears the display
    pub(crate) fn op_00e0(&mut self) {
        // self.display.iter_mut().for_each(|x| *x = false);//same result
        let l = self.display.len();
        let _ = &self.display[0..l].fill(false);
    }
    pub(crate) fn op_00ee(&mut self) {
        self.stack_ptr -= 1;
        self.program_counter = self.stack[self.stack_ptr as usize];
    }

    pub(crate) fn op_1nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.program_counter = address;
    }

    pub(crate) fn op_2nnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.stack[self.stack_ptr as usize] = self.program_counter;
        self.stack_ptr += 1;
        self.program_counter = address;
    }

    pub(crate) fn op_3xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;
        if self.v_registers[vx as usize] == byte as u8 {
            self.program_counter += 2;
        }
    }

    pub(crate) fn op_4xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;
        if self.v_registers[vx as usize] != byte {
            self.program_counter += 2;
        }
    }
    pub(crate) fn op_5xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        if self.v_registers[vx as usize] == self.v_registers[vy as usize] {
            self.program_counter += 2;
        }
    }
    pub(crate) fn op_9xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        if self.v_registers[vx as usize] != self.v_registers[vy as usize] {
            self.program_counter += 2;
        }
    }
    pub(crate) fn op_6xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;
        self.v_registers[vx as usize] = byte;
    }
    pub(crate) fn op_7xkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = (self.opcode & 0x00FF) as u8;
        let _ = self.v_registers[vx as usize].overflowing_add(byte);
    }
    pub(crate) fn op_8xy0(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        self.v_registers[vx as usize] = self.v_registers[vy as usize];
    }

    pub(crate) fn op_8xy1(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.v_registers[vx as usize] |= self.v_registers[vy as usize];
    }
    pub(crate) fn op_8xy2(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.v_registers[vx as usize] &= self.v_registers[vy as usize];
    }
    pub(crate) fn op_8xy3(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        self.v_registers[vx as usize] ^= self.v_registers[vy as usize];
    }
    pub(crate) fn op_8xy4(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        let (result, carry) =
            self.v_registers[vx as usize].overflowing_add(self.v_registers[vy as usize]);
        self.v_registers[0xF] = carry as u8;
        self.v_registers[vx as usize] = result;
    }
    pub(crate) fn op_8xy5(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        let (result, borrow) =
            self.v_registers[vx as usize].overflowing_sub(self.v_registers[vy as usize]);
        self.v_registers[0xF] = 1 - borrow as u8;
        self.v_registers[vx as usize] = result;
    }
    pub(crate) fn op_8xy6(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        self.v_registers[0xF] = self.v_registers[vx as usize] & 1;
        self.v_registers[vx as usize] >>= 1;
    }
    pub(crate) fn op_8xy7(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;

        let (result, borrow) =
            self.v_registers[vy as usize].overflowing_sub(self.v_registers[vx as usize]);
        self.v_registers[0xF] = 1 - borrow as u8;
        self.v_registers[vx as usize] = result;
    }
    pub(crate) fn op_8xye(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        self.v_registers[0xF] = (self.v_registers[vx as usize]) >> 7;
        self.v_registers[vx as usize] <<= 1;
    }

    pub(crate) fn op_annn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.i_register = address;
    }
    pub(crate) fn op_bnnn(&mut self) {
        let address = self.opcode & 0x0FFF;
        self.program_counter = self.v_registers[0] as u16 + address;
    }
    pub(crate) fn op_cxkk(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let byte = self.opcode & 0x00FF;

        let random: u8 = rand::rng().random();

        self.v_registers[vx as usize] = random & byte as u8;
    }

    pub(crate) fn op_dxyn(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let vy = (self.opcode & 0x00F0) >> 4;
        let height = self.opcode & 0x000F;

        let x_pos = self.v_registers[vx as usize] % PIXELS_WIDE as u8;
        let y_pos = self.v_registers[vy as usize] % PIXELS_HIGH as u8;
        self.v_registers[0xF] = 0;

        for i in 0..height {
            let sprite_byte = self.ram[i as usize + self.i_register as usize];
            for j in 0..8 {
                let sprite_pixel = (sprite_byte & (0x80 >> j)) != 0;
                let screen_index =
                    (y_pos as usize + i as usize) * PIXELS_WIDE + j as usize + x_pos as usize;
                let screen_pixel = &mut self.display[screen_index];

                if sprite_pixel {
                    if *screen_pixel {
                        self.v_registers[0xF] = 1;
                    }
                    *screen_pixel = !(*screen_pixel);
                }
            }
        }
    }

    pub(crate) fn op_ex9e(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let key = self.v_registers[vx as usize];
        if self.keys[key as usize] {
            self.program_counter += 2;
        }
    }
    pub(crate) fn op_exa1(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let key = self.v_registers[vx as usize];
        if !self.keys[key as usize] {
            self.program_counter += 2;
        }
    }

    pub(crate) fn op_fx07(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        self.v_registers[vx as usize] = self.delay_timer;
    }

    pub(crate) fn op_fx0a(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        for i in 0..=15 {
            if self.keys[i] {
                self.v_registers[vx as usize] = i as u8;
                return;
            }
        }
        self.program_counter -= 2;
    }

    pub(crate) fn op_fx15(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        self.delay_timer = self.v_registers[vx as usize];
    }

    pub(crate) fn op_fx18(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        self.sound_timer = self.v_registers[vx as usize];
    }

    pub(crate) fn op_fx1e(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        self.i_register = self
            .i_register
            .wrapping_add(self.v_registers[vx as usize] as u16);
    }

    pub(crate) fn op_fx29(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let digit = self.v_registers[vx as usize] as u16;

        self.i_register = FONTSET_ADDRESS_START as u16 + digit * 5;
    }

    pub(crate) fn op_fx33(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        let mut val = self.v_registers[vx as usize];

        self.ram[self.i_register as usize + 2] = val % 10;
        val /= 10;

        self.ram[self.i_register as usize + 1] = val % 10;
        val /= 10;

        self.ram[self.i_register as usize] = val % 10;
    }

    pub(crate) fn op_fx55(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;

        for i in 0..=vx {
            self.ram[self.i_register as usize + i as usize] = self.v_registers[i as usize];
        }
    }

    pub(crate) fn op_fx65(&mut self) {
        let vx = (self.opcode & 0x0F00) >> 8;
        for i in 0..=vx {
            self.v_registers[i as usize] = self.ram[self.i_register as usize + i as usize];
        }
    }

    fn extract_bits(val: u16, bits: u16, mask: u16) -> u8 {
        ((val & mask) >> bits) as u8
    }

    pub(crate) fn memory_dump(&mut self) {
        println!("MEMORY DUMP\n:");
        for i in 0..4095 {
            println!("Byte{}: {:#04x}", i, self.ram[i]);
        }
    }
}
