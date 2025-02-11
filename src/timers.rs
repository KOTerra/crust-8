use crate::cpu::Chip8Cpu;
use std::time::{Duration, Instant};

pub(crate) struct Timers {
    last_update: Instant,
}

impl Timers {
    pub(crate) fn new() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }

    pub(crate) fn update(&mut self, mut cpu: &mut Chip8Cpu) {
        let frame_duration: Duration = Duration::from_secs_f32(1.0 / 60.0);

        let now = Instant::now();
        if now.duration_since(self.last_update) >= frame_duration {
            // if cpu.delay_timer == 0 {
            //     cpu.delay_timer = 255;
            // } else {
            //     cpu.delay_timer -= 1;
            // }
            // if cpu.sound_timer == 0 {
            //     cpu.sound_timer = 255;
            // } else {
            //     cpu.sound_timer -= 1;
            // }
            if cpu.delay_timer > 0 {
                cpu.delay_timer -= 1;
            }
            if cpu.sound_timer > 0 {
                cpu.sound_timer -= 1;
            }
            self.last_update = now;
            // println!("Value: {}", cpu.delay_timer);
        }
    }
}
