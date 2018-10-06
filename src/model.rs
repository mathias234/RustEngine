extern crate glium;
extern crate tobj;

use colored::*;
use std::io::BufReader;

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub texcoord: [f32; 2],
}

implement_vertex!(ModelVertex, position, normal, tangent, texcoord);

pub struct Model {
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u16>,
    pub vertex_buffer: glium::VertexBuffer<ModelVertex>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub bounding_box: [f32; 3], // Size X, Size Y, Size Z
}

impl Model {
    pub fn load(display: &glium::Display, obj_buffer: &[u8], mtl_buffer: &[u8]) -> Model {
        let mut obj_buf = BufReader::new(obj_buffer);

        let tobj_model = tobj::load_obj_buf(&mut obj_buf, |_| {
            tobj::load_mtl_buf(&mut BufReader::new(mtl_buffer))
        });

        if !tobj_model.is_ok() {
            println!("{}", "Failed to load model".red());
        }

        let (models, _) = tobj_model.unwrap();

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
                tangent: [0.0, 0.0, 0.0],
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

        Model::calculate_tangents(&mut vertices, &indices);

        // println!("  Length of vertex array: {}", vertices.len());
        // println!("  Length of index array: {}", indices.len());

        let vb = glium::VertexBuffer::new(display, &vertices).unwrap();
        let ib = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        ).unwrap();

        let bounding_box = Model::calculate_bounding_box(&mut vertices);

        Model {
            vertices: vertices,
            indices: indices,
            vertex_buffer: vb,
            index_buffer: ib,
            bounding_box: bounding_box,
        }
    }

    fn calculate_bounding_box(vertices: &mut Vec<ModelVertex>) -> [f32; 3] {
        let mut min_x: f32 = 50000000.0;
        let mut max_x: f32 = -50000000.0;
        let mut min_y: f32 = 50000000.0;
        let mut max_y: f32 = -50000000.0;
        let mut min_z: f32 = 50000000.0;
        let mut max_z: f32 = -50000000.0;

        for i in 0..vertices.len() {
            let mut vpos = vertices[i].position;
            if vpos[0] < min_x {
                min_x = vpos[0];
            }
            if vpos[0] > max_x {
                max_x = vpos[0];
            }

            if vpos[1] < min_y {
                min_y = vpos[1];
            }
            if vpos[1] > max_y {
                max_y = vpos[1];
            }

            if vpos[2] < min_z {
                min_z = vpos[2];
            }
            if vpos[2] > max_z {
                max_z = vpos[2];
            }
        }

        let bounding_box = [
            min_x.abs() + max_x.abs(),
            min_y.abs() + max_y.abs(),
            min_z.abs() + max_z.abs(),
        ];

        bounding_box
    }

    pub fn sub_vec3(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
        [l[0] - r[0], l[1] - r[1], l[2] - r[2]]
    }

    pub fn add_vec3(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
        [l[0] + r[0], l[1] + r[1], l[2] + r[2]]
    }

    pub fn length_vec3(v: [f32; 3]) -> f32 {
        v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
    }

    pub fn normalize_vec3(v: [f32; 3]) -> [f32; 3] {
        let length = Model::length_vec3(v);
        [v[0] / length, v[1] / length, v[2] / length]
    }

    pub fn calculate_tangents(vertices: &mut Vec<ModelVertex>, indices: &Vec<u16>) {
        for i in 0..indices.len() / 3 {
            let index = i * 3;
            let i0 = indices[index] as usize;
            let i1 = indices[index + 1] as usize;
            let i2 = indices[index + 2] as usize;

            let edge1 = Model::sub_vec3(vertices[i1].position, vertices[i0].position);
            let edge2 = Model::sub_vec3(vertices[i2].position, vertices[i0].position);

            let delta_u1 = vertices[i1].texcoord[0] - vertices[i0].texcoord[0];
            let delta_v1 = vertices[i1].texcoord[1] - vertices[i0].texcoord[1];
            let delta_u2 = vertices[i2].texcoord[0] - vertices[i0].texcoord[0];
            let delta_v2 = vertices[i2].texcoord[1] - vertices[i0].texcoord[1];

            let dividend = delta_u1 * delta_v2 - delta_u2 * delta_v1;

            let mut f: f32;
            if dividend.abs() < 0.001 {
                f = 1.0;
            } else {
                f = 1.0 / dividend;
            }

            let tangent = [
                f * (delta_v2 * edge1[0] - delta_v1 * edge2[0]),
                f * (delta_v2 * edge1[1] - delta_v1 * edge2[1]),
                f * (delta_v2 * edge1[2] - delta_v1 * edge2[2]),
            ];

            vertices[i0].tangent = Model::add_vec3(vertices[i0].tangent, tangent);
            vertices[i1].tangent = Model::add_vec3(vertices[i1].tangent, tangent);
            vertices[i2].tangent = Model::add_vec3(vertices[i2].tangent, tangent);
        }

        for i in 0..vertices.len() {
            vertices[i].tangent = Model::normalize_vec3(vertices[i].tangent);
        }
    }
}
