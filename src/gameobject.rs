extern crate glium;
use math_helper;
use model::*;
use quaternion::Quaternion;

pub struct GameObject {
    pub name: String,
    pub model: Model,
    pub position: [f32; 3],
    pub rotation: Quaternion,
    pub shader_program: glium::Program,
    pub texture: glium::texture::SrgbTexture2d,
}

impl GameObject {
    pub fn new(
        name: String,
        position: [f32; 3],
        rotation: Quaternion,
        model: Model,
        shader_program: glium::Program,
        texture: glium::texture::SrgbTexture2d,
    ) -> GameObject {
        GameObject {
            name: name,
            position: position,
            rotation: rotation,
            model: model,
            shader_program: shader_program,
            texture: texture,
        }
    }

    pub fn rotate(&mut self, axis: [f32; 3], angle: f32) {
        let rot = Quaternion::new_axis_angle(axis, angle);

        let old_rot = self.rotation;

        self.rotation = rot.mul_quat(old_rot);
    }
    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        self.rotation.conjugate().to_rotation_matrix()
    }

    fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0],
        ]
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        math_helper::mat_mul(self.get_rotation_matrix(), self.get_translation_matrix())
    }
}
