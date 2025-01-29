pub(crate) struct Input {
    pub key_w: bool,
    pub key_s: bool,
}
impl Input {
    pub(crate) fn new() -> Input {
        Input {
            key_w: false,
            key_s: false,
        }
    }
    pub(crate) fn update(&mut self, event: &glium::winit::event::WindowEvent) {
        self.key_w = false;
        self.key_s = false;
        use glium::winit::keyboard::{KeyCode, PhysicalKey};
        let glium::winit::event::WindowEvent::KeyboardInput { event, .. } = event else {
            return;
        };
        let pressed = event.state == glium::winit::event::ElementState::Pressed;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW | KeyCode::ArrowUp) => self.key_w = pressed,
            PhysicalKey::Code(KeyCode::KeyS |KeyCode::ArrowDown) => self.key_s = pressed,
            _ => (),
        };
        if pressed {
            println!("w:{} s:{}", self.key_w, self.key_s);
        }
    }
}
