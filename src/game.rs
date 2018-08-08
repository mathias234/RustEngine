extern crate glium;

use gameobject::*;
use glium::glutin;
use model::*;
use physics_engine::PhysicsContext;
use quaternion::Quaternion;
use renderer::*;
use resource_manager::ResourceContext;
use shader;
use texture;
use vector::Vector3;

pub fn start(
    display: &glium::Display,
    context: &mut RenderContext,
    res: &mut ResourceContext,
    physics: &mut PhysicsContext,
) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let test_model = res.alloc_model(Model::load(&display, "res/test.obj".to_string()));
    let plane_model = res.alloc_model(Model::load(&display, "res/plane.obj".to_string()));

    let bricks = res.alloc_tex(texture::load(&display, "res/nicebrick.png".to_string()));
    let bricksnrm = res.alloc_tex(texture::load(&display, "res/nicebrick_nrm.png".to_string()));

    let grass = res.alloc_tex(texture::load(&display, "res/grass.png".to_string()));
    let grassnrm = res.alloc_tex(texture::load(&display, "res/grass_nrm.png".to_string()));

    let basic_shader = res.alloc_shader(shader::load(&display, "res/basic"));

    let plane = GameObject::new(
        physics,
        "plane".to_string(),
        Vector3::new(0.0, -5.0, 0.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        plane_model,
        basic_shader,
        grass,
        grassnrm,
        false,
    );

    let monkey_head = GameObject::new(
        physics,
        "monkey_head".to_string(),
        Vector3::new(0.0, 0.0, 10.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        test_model,
        basic_shader,
        bricks,
        bricksnrm,
        true,
    );

    let monkey_head2 = GameObject::new(
        physics,
        "monkey_head2".to_string(),
        Vector3::new(0.5, 2.0, 10.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        test_model,
        basic_shader,
        bricks,
        bricksnrm,
        true,
    );

    context.models.push(monkey_head);
    context.models.push(monkey_head2);
    context.models.push(plane)
}

static ANGLE: f32 = 0.5;

pub fn update(context: &mut RenderContext, delta_time: f32) {
    context.camera.update(delta_time);
    // let monkey_head = context.get_gameobject("monkey_head".to_string());
    // monkey_head.rotate(Vector3::new(0.0, 1.0, 0.0), ANGLE * delta_time);
    // monkey_head.rotate(Vector3::new(0.0, 0.0, 1.0), ANGLE * delta_time);
    // monkey_head.rotate(Vector3::new(1.0, 0.0, 0.0), ANGLE * delta_time);
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}

pub fn process_input_device(context: &mut RenderContext, event: &glutin::DeviceEvent) {
    context.camera.process_input_device(event);
}
