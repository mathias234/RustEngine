extern crate glium;
use math_helper;
use model::*;

pub struct GameObject {
    pub name: String,
    pub model: Model,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub shader_program: glium::Program,
    pub texture: glium::texture::Texture2d,
}

impl GameObject {
    pub fn new(
        name: String,
        position: [f32; 3],
        rotation: [f32; 3],
        model: Model,
        shader_program: glium::Program,
        texture: glium::texture::Texture2d,
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

    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        let cos_x = self.rotation[0].cos();
        let sin_x = self.rotation[0].sin();

        let cos_y = self.rotation[1].cos();
        let sin_y = self.rotation[1].sin();

        let cos_z = self.rotation[2].cos();
        let sin_z = self.rotation[2].sin();

        let rz = [
            [cos_z, sin_z, 0.0, 0.0],
            [-sin_z, cos_z, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let rx = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_x, sin_x, 0.0],
            [0.0, -sin_x, cos_x, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let ry = [
            [cos_y, 0.0, sin_y, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_y, 0.0, cos_y, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        math_helper::mat_mul(rz, math_helper::mat_mul(ry, rx))
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
