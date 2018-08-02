#[macro_use]
extern crate glium;
extern crate obj;

mod game;
mod renderer;
mod camera;

fn main() {
    use glium::{glutin};
    
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

        uniform mat4 persp_matrix;
        uniform mat4 view_matrix;

        in vec3 position;
        in vec3 normal;

        out vec3 _normal;

        void main() {
            gl_Position = persp_matrix * view_matrix * vec4(position, 1.0);
            _normal = normalize(normal);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        uniform vec3 light;

        in vec3 _normal;
        out vec4 result;


        void main() {
            result = vec4(clamp(dot(_normal, -light), 0.0f, 1.0f) * vec3(1.0f, 0.93f, 0.56f), 1.0f);            
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut render_context = renderer::init_renderer();

    render_context.program = Some(program);

    game::start(&display, &mut render_context);

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
                    ev => game::process_input(&mut render_context, &ev),
                },

                _ => (),
            }
        });
    }
}