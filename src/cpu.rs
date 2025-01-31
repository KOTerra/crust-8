use crate::input::Input;
use std::ffi::c_uchar;
use std::os::raw::c_int;

pub(crate) struct Chip8Cpu {
    registers: [u8; 16],
    i_register: u16,
    stack: [u16; 16],
    stack_ptr: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,

    ram: [u8; 4096],

    display: [bool; 64 * 32],
    draw_flag: bool,

    keys: [bool; 16],
}

impl Chip8Cpu {
    pub(crate) fn new() -> Self {
        let mut returned: Chip8Cpu = Self {
            registers: [0; 16],
            i_register: 0,
            stack: [0; 16],
            stack_ptr: 0,
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            ram: [0; 4096],
            display: [false; 64 * 32],
            draw_flag: false,
            keys: [false; 16],
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
}
