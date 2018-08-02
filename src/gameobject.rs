
extern crate glium;
use model::*;

pub struct GameObject {
    pub model: Model,
    pub position: [f32; 3],
    pub shader_program: glium::Program,
}

impl GameObject {
    pub fn new(position: [f32; 3], model: Model, shader_program: glium::Program) -> GameObject {
        GameObject {position: position, model: model, shader_program: shader_program }
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position[0], self.position[1], self.position[2], 1.0],
        ]
    }
}