extern crate glium;
extern crate obj;

use std::fs::File;
use std::io::BufReader;
use obj::*;

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(ModelVertex, position, normal);


pub struct Model {
    pub vertex_buffer: glium::VertexBuffer<ModelVertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

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
