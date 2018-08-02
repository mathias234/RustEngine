extern crate glium;

pub struct RenderContext {
    pub clear_r : f32,
    pub clear_g : f32,
    pub clear_b : f32,
    pub models: Vec<Model>,
    pub program: Option<glium::Program>,
}

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {clear_r: 0.0, clear_g: 0.0, clear_b: 0.0, models: Vec::new(), program: None}
    }
}

pub fn init_renderer() -> RenderContext {
    implement_vertex!(Vertex, position);
    
    RenderContext::new()
}

pub fn update_renderer(context: &mut RenderContext, display: &glium::Display, target: &mut glium::Frame) {
    use glium::{glutin, Surface};

    target.clear_color(context.clear_r, context.clear_g, context.clear_b, 1.0);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    for i in 0..context.models.len() {
        let mut vertex_buffer = glium::VertexBuffer::new(display, &context.models[i].vertices).unwrap();

        if(context.program.is_none()) {
            println!("No shader program available");
        }
        else {
            let prog = context.program.take().unwrap();

            target.draw(&vertex_buffer, &indices,  &prog, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

            context.program = Some(prog);
        }
    }
}
