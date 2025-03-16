//https://glium.github.io/glium/book/tuto-04-matrices.html
#[macro_use]
extern crate glium;
mod cpu;
mod input;
mod timers;
mod utils;

use crate::cpu::Chip8Cpu;
use crate::input::Input;
use crate::timers::Timers;
use glium::winit::event;
use glium::Surface;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Crust-8")
        .build(&event_loop);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.8, 1.0, 0.8, 1.0);       //color of pixels
        }
    "#;
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut input = Input::new();
    input.file_name = String::from("roms/3-corax+.ch8");
    let mut cpu = Chip8Cpu::new();
    cpu.open_rom(&input);
    let mut timer = Timers::new();
    

    //size of each square in the grid normalized  (OpenGL coordinate system)
    let square_width = 2.0 / 64.0;
    let square_height = 2.0 / 32.0;

    let mut grid: [[bool; 64]; 32] = [[false; 64]; 32];
    cpu.draw_flag=true;


    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            //sau la final daca nu merge input
            cpu.execute_cycle();      
            timer.update(&mut cpu);

            //treat the event
            match ev {
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    event::WindowEvent::DroppedFile(path) => {
                        //TODO reinitialize cpu with new path
                        input.file_name = String::from(path.to_str().unwrap());
                        cpu.reset();
                        cpu.open_rom(&input);
                    }
                    event::WindowEvent::KeyboardInput {
                        device_id: _,
                        event: _,
                        is_synthetic: _,
                    } => {
                        input.update(&event);
                        cpu.recieve_input(&input);
                        utils::fill_matrix_random(&mut grid);
                        if input.key_memory_dump {
                            cpu.memory_dump();
                        }
                        // if input.key_draw_flag {
                        //     cpu.draw_flag = true;
                        // } else {
                        //     cpu.draw_flag = false;
                        // }
                    }

                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    event::WindowEvent::RedrawRequested => {
                        //idk
                        use std::time::{Duration, Instant};
                        let frame_duration: Duration = Duration::from_secs_f32(1.0 / 60.0); // 60 FPS

                        let mut last_frame_time: Option<Instant> = None;

                        let now = Instant::now();
                        if let Some(last_time) = last_frame_time {
                            if now.duration_since(last_time) < frame_duration {
                                return; // Skip this frame if it's too soon
                            }
                        }
                        last_frame_time = Some(now); // Update last frame time
                                                     //idk

                        if cpu.draw_flag {
                            //sau la inceput TODO
                            utils::copy_array(&mut grid, &mut cpu.display);
                            let mut target = display.draw();
                            target.clear_color(0.0, 0.01, 0.0, 1.0);

                            for (row_idx, row) in grid.iter().enumerate() {
                                for (col_idx, &cell) in row.iter().enumerate() {
                                    if cell {
                                        let x = -1.0 + (col_idx as f32 * square_width);
                                        let y = 1.0 - (row_idx as f32 * square_height);

                                        let vertices = [
                                            Vertex { position: [x, y] },
                                            Vertex {
                                                position: [x + square_width, y],
                                            },
                                            Vertex {
                                                position: [x, y - square_height],
                                            },
                                            Vertex {
                                                position: [x + square_width, y - square_height],
                                            },
                                        ];

                                        let vertex_buffer =
                                            glium::VertexBuffer::new(&display, &vertices).unwrap();
                                        let indices = glium::index::NoIndices(
                                            glium::index::PrimitiveType::TriangleStrip,
                                        );

                                        let uniforms = uniform! {
                                            matrix: [
                                                [1.0, 0.0, 0.0, 0.0],
                                                [0.0, 1.0, 0.0, 0.0],
                                                [0.0, 0.0, 1.0, 0.0],
                                                [0.0, 0.0, 0.0, 1.0f32],
                                            ]
                                        };

                                        target
                                            .draw(
                                                &vertex_buffer,
                                                &indices,
                                                &program,
                                                &uniforms,
                                                &Default::default(),
                                            )
                                            .unwrap();
                                    }
                                }
                            }

                            target.finish().unwrap();
                        }
                    }

                    event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },

                event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
            // cpu.execute_cycle();
            // timer.update(&mut cpu);
        })
        .unwrap();

    println!("stop");
}
