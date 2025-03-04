use crate::input::Input;
use std::fs::File;
use std::io::{BufReader, Read};

pub(crate) struct Chip8Cpu {
    v_registers: [u8; 16], //v[0] - v[F], F is flags register
    i_register: u16,
    stack: Vec<u16>, //size 16
    stack_ptr: u16,
    program_counter: u16,
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,

    ram: [u8; 4096],

    pub(crate) display: [bool; 64 * 32],
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
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; 4096],
            display: [false; 64 * 32],
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
            returned.ram[i] = font[i];
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

    pub(crate) fn execute_cycle(&mut self) {
        self.fetch();
        if (self.program_counter as usize) < 4094 {
            self.program_counter += 2;
        }
        self.decode();

        self.execute();
    }
    fn fetch(&mut self) {
        self.opcode = (self.ram[self.program_counter as usize] as u16) << 8;
        self.opcode = self.opcode | (self.ram[self.program_counter as usize + 1] as u16);
    }
    fn decode(&mut self) {}
    fn execute(&mut self) {}

    //clears the display
    pub(crate) fn op_00E0(&mut self) {
        // self.display.iter_mut().for_each(|x| *x = false);//same result
        let l = self.display.len();
        &self.display[0..l].fill(false);
    }
    pub(crate) fn op_00EE(&mut self) {
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
        self.v_registers[vx as usize] += byte;
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

        self.v_registers[0xF] = (self.opcode & 0x0001) as u8;
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

    pub(crate) fn op_annn(&mut self) {}

    fn extract_bits(val: u16, bits: u16, mask: u16) -> u8 {
        ((val & mask) >> bits) as u8
    }

    pub(crate) fn open_rom(&mut self, input: &Input) {
        let mut f = BufReader::new(File::open(input.file_name.clone()).expect("open failed"));

        let mut buf = vec![0u8; 4096 - 512];

        loop {
            let bytes_read = f.read(&mut buf).expect("read failed");
            if bytes_read == 0 {
                break;
            }

            let mut start_index = 0x200; //512

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
        self.program_counter = 0x200;
        self.stack = vec![0];
        self.stack_ptr = 0;
        self.opcode = 0;
        &self.ram[self.program_counter as usize..self.program_counter as usize + 512].fill(0x00);
    }

    pub(crate) fn memory_dump(&mut self) {
        println!("MEMORY DUMP\n:");
        for i in 0..4095 {
            println!("Byte{}: {:#04x}", i, self.ram[i]);
        }
    }
}
