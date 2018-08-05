extern crate glium;
use math_helper;
use model::*;
use quaternion::Quaternion;
use vector::Vector3;

pub struct GameObject {
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub model: usize,
    pub shader_program: usize,
    pub texture: usize,
    pub normal_map: usize,
}

impl GameObject {
    pub fn new(
        name: String,
        position: Vector3,
        rotation: Quaternion,
        model: usize,
        shader_program: usize,
        texture: usize,
        normal_map: usize,
    ) -> GameObject {
        GameObject {
            name: name,
            position: position,
            rotation: rotation,
            model: model,
            shader_program: shader_program,
            texture: texture,
            normal_map: normal_map,
        }
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        let rot = Quaternion::new_axis_angle(axis, angle);

        let old_rot = self.rotation;

        self.rotation = rot.mul_quat(old_rot);
    }

    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        self.rotation.to_rotation_matrix()
    }

    fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position.x, self.position.y, self.position.z, 1.0],
        ]
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        math_helper::mat_mul(self.get_rotation_matrix(), self.get_translation_matrix())
    }
}
