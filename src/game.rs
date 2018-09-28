extern crate glium;

use gameobject::*;
use glium::glutin;
use material::*;
use model::*;
use physics_engine::{PhysicsContext, PhysicsShape};
use quaternion::Quaternion;
use renderer::*;
use resource_manager::ResourceContext;
use shader;
use texture;
use ui_renderer::*;
use vector::Vector3;

pub fn start(
    display: &glium::Display,
    context: &mut RenderContext,
    res: &mut ResourceContext,
    physics: &mut PhysicsContext,
) {
    context.clear_color = [0.0, 0.3, 0.7];

    let test_model = res.alloc_model(Model::load(&display, "res/test.obj".to_string()));
    let plane_model = res.alloc_model(Model::load(&display, "res/plane.obj".to_string()));

    let bricks = res.alloc_tex(texture::load(&display, "res/nicebrick.jpg".to_string()));
    let bricksnrm = res.alloc_tex(texture::load(&display, "res/nicebrick_nrm.jpg".to_string()));

    let grass = res.alloc_tex(texture::load(&display, "res/grass.jpg".to_string()));
    let grassnrm = res.alloc_tex(texture::load(&display, "res/grass_nrm.jpg".to_string()));

    let basic_shader = res.alloc_shader(shader::load(&display, "res/basic"));

    let grass_material = Material::new(basic_shader, grass, grassnrm, [1.0, 1.0, 1.0], [3.0, 3.0]);
    let brick_material =
        Material::new(basic_shader, bricks, bricksnrm, [1.0, 1.0, 1.0], [1.0, 1.0]);

    let ground_material =
        Material::new(basic_shader, bricks, bricksnrm, [1.0, 1.0, 1.0], [5.0, 5.0]);

    let plane = GameObject::new(
        res,
        "plane".to_string(),
        Vector3::new(0.0, -5.0, 0.0),
        Quaternion::new(0.0, 0.0, 0.0, 1.0),
        plane_model,
        ground_material,
    ).add_collider(physics, PhysicsShape::BoxShape);
    let plane = res.alloc_gameobject(plane);
    context.gameobjects.push(plane);

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..5 {
                let a = (x + y + z) % 2;

                if a == 1 {
                    let sphere = GameObject::new(
                        res,
                        "sphere".to_string(),
                        Vector3::new(x as f32, 10.0 + y as f32, z as f32),
                        Quaternion::new(0.0, 0.0, 0.0, 1.0),
                        test_model,
                        grass_material,
                    ).add_rigidbody(physics, PhysicsShape::SphereShape);
                    let sphere = res.alloc_gameobject(sphere);
                    context.gameobjects.push(sphere);
                } else {
                    let sphere = GameObject::new(
                        res,
                        "sphere".to_string(),
                        Vector3::new(x as f32, 10.0 + y as f32, z as f32),
                        Quaternion::new(0.0, 0.0, 0.0, 1.0),
                        test_model,
                        brick_material,
                    ).add_rigidbody(physics, PhysicsShape::SphereShape);
                    let sphere = res.alloc_gameobject(sphere);
                    context.gameobjects.push(sphere);
                }
            }
        }
    }
}

pub fn update(context: &mut RenderContext, delta_time: f32) {
    context.camera.update(delta_time);
}

pub fn render_gui(ui: &mut UIContext) {
    ui.render_quad(0.5, 0.5, 50.0, 50.0);
}

pub fn process_input(context: &mut RenderContext, event: &glutin::WindowEvent) {
    context.camera.process_input(event);
}

pub fn process_input_device(context: &mut RenderContext, event: &glutin::DeviceEvent) {
    context.camera.process_input_device(event);
}
