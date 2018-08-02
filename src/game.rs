extern crate glium;

use renderer::*;
use model::*;
use glium::{glutin};
use gameobject::*;


pub fn start(display: &glium::Display, context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let monkey_head_model = Model::load_model("test.obj".to_string(), &display);

    let monkey_head = GameObject::new([0.0, 0.0, -10.0], monkey_head_model);

    context.models.push(monkey_head);
}

pub fn update(context: &mut RenderContext) {
    context.camera.update();
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}