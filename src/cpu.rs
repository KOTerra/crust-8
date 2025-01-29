use crate::input::Input;

pub(crate) struct chip8_cpu {
    registers: [u8; 16],
}

impl chip8_cpu {
    pub(crate) fn new() -> Self {
        Self { registers: [0; 16] }
    }
    pub(crate) fn recieve_input(&mut self, input: &Input) {}
}
