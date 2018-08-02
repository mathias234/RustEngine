#[macro_use]
extern crate glium;
extern crate obj;

mod game;
mod renderer;
mod camera;
mod model;
mod gameobject;
mod shader;

fn main() {
    use glium::{glutin};
    
    // Create A window
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()      
        .with_title("Rust Engine")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let glutin_context = glutin::ContextBuilder::new().with_vsync(true);
    
    let display = glium::Display::new(window, glutin_context, &events_loop).unwrap();

    // #[derive(Copy, Clone)]
    // struct Vertex {
    //     position: [f32; 2],
    // }

    // let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = shader::read_shader_file(&display, "basic");

    let mut render_context = renderer::init_renderer();

    render_context.program = Some(program);

    game::start(&display, &mut render_context);

    // Game Loop
    let mut closed = false;
    while !closed {
        game::update(&mut render_context);

        let mut target = display.draw();

        renderer::update_renderer(&mut render_context, &mut target);
        
        // target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    ev => game::process_input(&mut render_context, &ev),
                },

                _ => (),
            }

        });
    }
}