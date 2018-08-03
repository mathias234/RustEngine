extern crate glium;
extern crate tobj;
use camera::*;
use gameobject::*;

pub struct RenderContext {
    pub clear_r : f32,
    pub clear_g : f32,
    pub clear_b : f32,
    pub models: Vec<GameObject>,
    pub camera: CameraState,
}

impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {camera: CameraState::new(), clear_r: 0.0, clear_g: 0.0, clear_b: 0.0, models: Vec::new()}
    }

    pub fn get_gameobject(&mut self, name: String) -> &mut GameObject {
        for i in 0..self.models.len() {
            if self.models[i].name == name {
                return &mut self.models[i];
            }
        }

        &mut self.models[0]
    }
}

pub fn init_renderer() -> RenderContext {
    RenderContext::new()
}

pub fn update_renderer(context: &mut RenderContext, target: &mut glium::Frame) {
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

    let pers_mat = context.camera.get_perspective();
    let view_mat = context.camera.get_view();

    for i in 0..context.models.len() {
        let gobj = &context.models[i];
        let program = &gobj.shader_program;


        let model = &gobj.model;

        let model_matrix = gobj.get_model_matrix();

        let uniforms = uniform! {
            persp_matrix: pers_mat,
            view_matrix: view_mat,
            model_matrix: model_matrix,
            light: (-1.0, -1.0, -1.0f32),
            ambient_light: 0.2 as f32,
        };

        target.draw(&model.vertex_buffer, &model.index_buffer, &program, &uniforms, &params).unwrap();
    }
}
