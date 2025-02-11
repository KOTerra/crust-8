pub(crate) struct Input {
    pub file_name: String,
    pub key_1: bool,
    pub key_2: bool,
    pub key_3: bool,
    pub key_4: bool,
    pub key_q: bool,
    pub key_w: bool,
    pub key_e: bool,
    pub key_r: bool,
    pub key_a: bool,
    pub key_s: bool,
    pub key_d: bool,
    pub key_f: bool,
    pub key_z: bool,
    pub key_x: bool,
    pub key_c: bool,
    pub key_v: bool,
    pub key_memory_dump: bool,
    pub(crate) key_draw_flag: bool,
}
impl Input {
    pub(crate) fn new() -> Input {
        Input {
            file_name: String::new(),
            key_1: false,
            key_2: false,
            key_3: false,
            key_4: false,
            key_q: false,
            key_w: false,
            key_e: false,
            key_r: false,
            key_a: false,
            key_s: false,
            key_d: false,
            key_f: false,
            key_z: false,
            key_x: false,
            key_c: false,
            key_v: false,
            key_memory_dump: false,
            key_draw_flag: false,
        }
    }
    pub(crate) fn update(&mut self, event: &glium::winit::event::WindowEvent) {
        use glium::winit::keyboard::{KeyCode, PhysicalKey};

        let glium::winit::event::WindowEvent::KeyboardInput { event, .. } = event else {
            return;
        };

        let pressed = event.state == glium::winit::event::ElementState::Pressed;

        match &event.physical_key {
            PhysicalKey::Code(KeyCode::Digit1) => self.key_1 = pressed,
            PhysicalKey::Code(KeyCode::Digit2) => self.key_2 = pressed,
            PhysicalKey::Code(KeyCode::Digit3) => self.key_3 = pressed,
            PhysicalKey::Code(KeyCode::Digit4) => self.key_4 = pressed,
            PhysicalKey::Code(KeyCode::KeyQ) => self.key_q = pressed,
            PhysicalKey::Code(KeyCode::KeyW) => self.key_w = pressed,
            PhysicalKey::Code(KeyCode::KeyE) => self.key_e = pressed,
            PhysicalKey::Code(KeyCode::KeyR) => self.key_r = pressed,
            PhysicalKey::Code(KeyCode::KeyA) => self.key_a = pressed,
            PhysicalKey::Code(KeyCode::KeyS) => self.key_s = pressed,
            PhysicalKey::Code(KeyCode::KeyD) => self.key_d = pressed,
            PhysicalKey::Code(KeyCode::KeyF) => self.key_f = pressed,
            PhysicalKey::Code(KeyCode::KeyZ) => self.key_z = pressed,
            PhysicalKey::Code(KeyCode::KeyX) => self.key_x = pressed,
            PhysicalKey::Code(KeyCode::KeyC) => self.key_c = pressed,
            PhysicalKey::Code(KeyCode::KeyV) => self.key_v = pressed,
            PhysicalKey::Code(KeyCode::KeyI) => self.key_memory_dump = pressed,
            PhysicalKey::Code(KeyCode::KeyO) => self.key_draw_flag = pressed,
            _ => (),
        };
        let mut active_keys = Vec::new();

        if self.key_w {
            active_keys.push("W");
        }
        if self.key_s {
            active_keys.push("S");
        }
        if self.key_a {
            active_keys.push("A");
        }
        if self.key_d {
            active_keys.push("D");
        }
        if self.key_q {
            active_keys.push("Q");
        }
        if self.key_e {
            active_keys.push("E");
        }
        if self.key_r {
            active_keys.push("R");
        }
        if self.key_z {
            active_keys.push("Z");
        }
        if self.key_x {
            active_keys.push("X");
        }
        if self.key_c {
            active_keys.push("C");
        }
        if self.key_v {
            active_keys.push("V");
        }

        if !active_keys.is_empty() {
            println!("Keys: {}", active_keys.join(", "));
        }
    }
}
