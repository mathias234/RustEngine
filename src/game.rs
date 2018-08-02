extern crate glium;

use renderer::{RenderContext, Model};
use glium::{glutin};


pub fn start(display: &glium::Display, context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    context.models.push(Model::load_model("test.obj".to_string(), &display))
}

pub fn update(context: &mut RenderContext) {
    context.camera.update();
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}