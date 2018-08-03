extern crate glium;

use gameobject::*;
use glium::glutin;
use model::*;
use renderer::*;
use shader;
use texture;

pub fn start(display: &glium::Display, context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let monkey_head_model = Model::load_model(&display, "res/test.obj".to_string());
    let plane = Model::load_model(&display, "res/plane.obj".to_string());

    let plane = GameObject::new(
        "plane".to_string(),
        [0.0, -2.0, 0.0],
        [0.0, 0.0, 0.0],
        plane,
        shader::read_shader_file(&display, "res/basic"),
        texture::load_texture(&display, "res/grass.png".to_string()),
    );

    let monkey_head = GameObject::new(
        "monkey_head".to_string(),
        [0.0, 0.0, -10.0],
        [0.0, 0.0, 0.0],
        monkey_head_model,
        shader::read_shader_file(&display, "res/basic"),
        texture::load_texture(&display, "res/bricks.jpg".to_string()),
    );

    context.models.push(monkey_head);
    context.models.push(plane)
}

static ANGLE: f32 = 0.5;

pub fn update(context: &mut RenderContext, delta_time: f32) {
    context.camera.update();
    let monkey_head = context.get_gameobject("monkey_head".to_string());
    monkey_head.rotation[1] += ANGLE * delta_time;
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}
