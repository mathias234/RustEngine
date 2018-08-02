#[macro_use]
extern crate glium;
mod game;
mod renderer;

fn main() {
    use glium::{glutin, Surface};
    
    // Create A window
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // #[derive(Copy, Clone)]
    // struct Vertex {
    //     position: [f32; 2],
    // }

    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut render_context = renderer::init_renderer();

    render_context.program = Some(program);

    game::start(&mut render_context);

    // Game Loop
    let mut closed = false;
    while !closed {
        game::update(&mut render_context);

        let mut target = display.draw();

        renderer::update_renderer(&mut render_context, &display, &mut target);
        
        // target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },

                _ => (),
            }
        });
    }
}