extern crate glium;
extern crate obj;

use camera::*;
use model::*;

pub struct RenderContext {
    pub clear_r : f32,
    pub clear_g : f32,
    pub clear_b : f32,
    pub models: Vec<Model>,
    pub program: Option<glium::Program>,
    pub camera: CameraState,
}


impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {camera: CameraState::new(), clear_r: 0.0, clear_g: 0.0, clear_b: 0.0, models: Vec::new(), program: None}
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
