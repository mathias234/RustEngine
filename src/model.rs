extern crate glium;
extern crate tobj;

use std::path::Path;

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}

implement_vertex!(ModelVertex, position, normal, texcoord);

pub struct Model {
    pub vertex_buffer: glium::VertexBuffer<ModelVertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
}

impl Model {
    pub fn load_model(display: &glium::Display, filename: String) -> Model {
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
                normal: [0.0, 0.0, 0.0],
                texcoord: [0.0, 0.0],
            });

            // normals are optional
            if mesh.normals.len() > 0 {
                vertices[v].normal = [
                    mesh.normals[3 * v],
                    mesh.normals[3 * v + 1],
                    mesh.normals[3 * v + 2],
                ];
            }

            // tex coords are optional
            if mesh.texcoords.len() > 0 {
                vertices[v].texcoord = [mesh.texcoords[2 * v], mesh.texcoords[2 * v + 1]];
            }
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
