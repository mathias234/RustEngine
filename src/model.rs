extern crate glium;
extern crate tobj;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    pub fn load_model(filename: String, display: &glium::Display) -> Model {
        use glium::GlObject;

        println!("Loading model: {}", filename);

        let tobj_model = tobj::load_obj(&Path::new(&filename));
        assert!(tobj_model.is_ok());

        let (models, materials) = tobj_model.unwrap();

        let mesh = &models[0].mesh;

        let mut vertices: Vec<ModelVertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        for i in 0..mesh.indices.len() {
            indices.push(mesh.indices[i] as u16);
        }

        for v in 0..mesh.positions.len() / 3 {
            vertices.push(ModelVertex {
                position: [
                    mesh.positions[3 * v],
                    mesh.positions[3 * v + 1],
                    mesh.positions[3 * v + 2],
                ],
                normal: [
                    mesh.normals[3 * v],
                    mesh.normals[3 * v + 1],
                    mesh.normals[3 * v + 2],
                ],
            });
        }

        println!("  Length of vertex array: {}", vertices.len());
        println!("  Length of index array: {}", indices.len());

        let vb = glium::VertexBuffer::new(display, &vertices).unwrap();
        let ib = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        println!(
            "Successfully loaded model, (Vertex ID: {}), (Index ID: {})",
            vb.get_id(),
            ib.get_id()
        );

        Model {
            vertex_buffer: vb,
            index_buffer: ib,
        }
    }
}
