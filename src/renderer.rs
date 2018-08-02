extern crate glium;
extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::*;
use camera::*;

pub struct RenderContext {
    pub clear_r : f32,
    pub clear_g : f32,
    pub clear_b : f32,
    pub models: Vec<Model>,
    pub program: Option<glium::Program>,
    pub camera: CameraState,
}

pub struct Model {
    pub vertex_buffer: glium::VertexBuffer<ModelVertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(ModelVertex, position, normal);

impl Model {
    fn convert_vertices(vertices: Vec<Vertex>) -> Vec<ModelVertex> {
        let mut new_vertices : Vec<ModelVertex> = Vec::new();

        for i in 0..vertices.len() {
            let old_vertex = &vertices[i];
            new_vertices.push(ModelVertex { position: old_vertex.position, normal: old_vertex.normal });
        }

        new_vertices
    }

    pub fn load_model(filename: String, display: &glium::Display) -> Model {
        use glium::GlObject;

        println!("Loading model: {}", filename);

        let input = BufReader::new(File::open(filename).unwrap());
        let obj: Obj = load_obj(input).unwrap();

        println!("  Length of vertex array: {}", obj.vertices.len());
        println!("  Length of index array: {}", obj.indices.len());

        let vb = glium::VertexBuffer::new(display, &Model::convert_vertices(obj.vertices)).unwrap();
        let ib = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &obj.indices).unwrap();

        println!("Successfully loaded model, (Vertex ID: {}), (Index ID: {})", vb.get_id(), ib.get_id());


        Model {vertex_buffer: vb, index_buffer: ib}
    }

}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {camera: CameraState::new(), clear_r: 0.0, clear_g: 0.0, clear_b: 0.0, models: Vec::new(), program: None}
    }
}

pub fn init_renderer() -> RenderContext {
    RenderContext::new()
}

pub fn update_renderer(context: &mut RenderContext, display: &glium::Display, target: &mut glium::Frame) {
    use glium::{Surface};

    target.clear_color_and_depth((context.clear_r, context.clear_g, context.clear_b, 1.0),1.0);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let uniforms = uniform! {
        persp_matrix: context.camera.get_perspective(),
        view_matrix: context.camera.get_view(),
        light: (-1.0, -1.0, -1.0f32)
    };

    for i in 0..context.models.len() {
        if context.program.is_none() {
            println!("No shader program available");
        }
        else {
            let program = context.program.take().unwrap();

            let model = &context.models[i];

            target.draw(&model.vertex_buffer, &model.index_buffer, &program, &uniforms, &params).unwrap();

            context.program = Some(program);
        }
    }
}
