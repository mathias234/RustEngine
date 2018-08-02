extern crate glium;
extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::*;

pub struct RenderContext {
    pub clear_r : f32,
    pub clear_g : f32,
    pub clear_b : f32,
    pub models: Vec<Model>,
    pub program: Option<glium::Program>,
}

pub struct Model {
    pub vertex_buffer: glium::VertexBuffer<obj::Vertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(ModelVertex, position, normal);

impl Model {
    fn ConvertVertices(vertices: Vec<Vertex>) -> Vec<ModelVertex> {
        Vec::new()
    }

    pub fn load_model(filename: String, display: &glium::Display) {

        let input = BufReader::new(File::open(filename).unwrap());
        let obj: Obj = load_obj(input).unwrap();


        let vb = glium::VertexBuffer::new(display, &Model::ConvertVertices(obj.vertices));
        let ib = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &obj.indices);
    }

}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {clear_r: 0.0, clear_g: 0.0, clear_b: 0.0, models: Vec::new(), program: None}
    }
}

pub fn init_renderer() -> RenderContext {
    RenderContext::new()
}

pub fn update_renderer(context: &mut RenderContext, display: &glium::Display, target: &mut glium::Frame) {
    use glium::{glutin, Surface};

    target.clear_color(context.clear_r, context.clear_g, context.clear_b, 1.0);


    for i in 0..context.models.len() {
        if(context.program.is_none()) {
            println!("No shader program available");
        }
        else {
            let prog = context.program.take().unwrap();

            let model = context.models[i];

            target.draw(&model.vertex_buffer, &model.index_buffer,  &prog, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

            context.program = Some(prog);
        }
    }
}
