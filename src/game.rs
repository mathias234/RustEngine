extern crate glium;

use gameobject::*;
use glium::glutin;
use model::*;
use quaternion::Quaternion;
use renderer::*;
use shader;
use texture;
use vector::Vector3;

pub fn start(display: &glium::Display, context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let monkey_head_model = Model::load_model(&display, "res/test.obj".to_string());
    let plane = Model::load_model(&display, "res/plane.obj".to_string());

    let plane = GameObject::new(
        "plane".to_string(),
        Vector3::new(0.0, -2.0, 0.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        plane,
        shader::read_shader_file(&display, "res/basic"),
        texture::load_texture(&display, "res/grass.png".to_string()),
    );

    let monkey_head = GameObject::new(
        "monkey_head".to_string(),
        Vector3::new(0.0, 0.0, -10.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        monkey_head_model,
        shader::read_shader_file(&display, "res/basic"),
        texture::load_texture(&display, "res/bricks.jpg".to_string()),
    );

    context.models.push(monkey_head);
    context.models.push(plane)
}

static ANGLE: f32 = 0.5;

pub fn update(context: &mut RenderContext, delta_time: f32) {
    context.camera.update(delta_time);
    let monkey_head = context.get_gameobject("monkey_head".to_string());
    monkey_head.rotate(Vector3::new(0.0, 1.0, 0.0), ANGLE * delta_time);
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}

pub fn process_input_device(context: &mut RenderContext, event: &glutin::DeviceEvent) {
    context.camera.process_input_device(event);
}
