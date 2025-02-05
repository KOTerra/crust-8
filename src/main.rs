mod cpu;
mod input;
mod utils;

//https://glium.github.io/glium/book/tuto-04-matrices.html
#[macro_use]
extern crate glium;

use crate::cpu::Chip8Cpu;
use crate::input::Input;
use glium::winit::event;
use glium::winit::keyboard::{KeyCode, PhysicalKey};
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
    input.file_name = String::from("roms/PONG");
    let mut cpu = Chip8Cpu::new();
    cpu.open_rom(&input);

    // Define the size of each square in the grid
    let square_width = 2.0 / 64.0; // Normalized width (assuming OpenGL coordinate system)
    let square_height = 2.0 / 32.0; // Normalized height

    // Assuming you have a matrix of booleans
    let mut grid: [[bool; 64]; 32] = [[false; 64]; 32]; // Replace with your own matrix data

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    implement_vertex!(Vertex, position);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::CloseRequested => {
                        window_target.exit();
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
                    }

                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        target.clear_color(0.0, 0.01, 0.0, 1.0);

                        for (row_idx, row) in grid.iter().enumerate() {
                            for (col_idx, &cell) in row.iter().enumerate() {
                                if cell {
                                    // Only draw squares for `true` values
                                    let x = -1.0 + (col_idx as f32 * square_width);
                                    let y = 1.0 - (row_idx as f32 * square_height);

                                    // Define the square's vertices
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

                                    // Create vertex buffer
                                    let vertex_buffer =
                                        glium::VertexBuffer::new(&display, &vertices).unwrap();

                                    // Define indices for a triangle strip
                                    let indices = glium::index::NoIndices(
                                        glium::index::PrimitiveType::TriangleStrip,
                                    );

                                    // Uniform matrix (identity, as we don’t need transformations per square)
                                    let uniforms = uniform! {
                                        matrix: [
                                            [1.0, 0.0, 0.0, 0.0],
                                            [0.0, 1.0, 0.0, 0.0],
                                            [0.0, 0.0, 1.0, 0.0],
                                            [0.0, 0.0, 0.0, 1.0f32],
                                        ]
                                    };

                                    // Draw the square
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

                    event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
    println!("stop");
}
