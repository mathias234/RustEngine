extern crate glium;

use gameobject::*;
use glium::glutin;
use model::*;
use renderer::*;
use shader;

pub fn start(display: &glium::Display, context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let program = shader::read_shader_file(&display, "basic");

    let monkey_head_model = Model::load_model("test.obj".to_string(), &display);

    let monkey_head = GameObject::new(
        "monkey_head".to_string(),
        [0.0, 0.0, -10.0],
        [0.0, 0.0, 0.0],
        monkey_head_model,
        program,
    );

    context.models.push(monkey_head);
}

static ANGLE: f32 = 0.05;

pub fn update(context: &mut RenderContext) {
    context.camera.update();
    let monkey_head = context.get_gameobject("monkey_head".to_string());
    monkey_head.rotation[1] += ANGLE;
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}
