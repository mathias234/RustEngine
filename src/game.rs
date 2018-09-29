extern crate glium;
extern crate rand;

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

pub struct GameState {
    bricks: usize,
    grass: usize,
    text_size: f32,
}

impl GameState {
    pub fn start(
        display: &glium::Display,
        context: &mut RenderContext,
        res: &mut ResourceContext,
        physics: &mut PhysicsContext,
    ) -> GameState {
        context.clear_color = [0.0, 0.3, 0.7];

        let test_model = res.alloc_model(Model::load(&display, "res/test.obj".to_string()));
        let plane_model = res.alloc_model(Model::load(&display, "res/plane.obj".to_string()));

        let bricks = res.alloc_tex(texture::load(&display, "res/nicebrick.jpg".to_string()));
        let bricksnrm = res.alloc_tex(texture::load(&display, "res/nicebrick_nrm.jpg".to_string()));

        let grass = res.alloc_tex(texture::load(&display, "res/grass.jpg".to_string()));
        let grassnrm = res.alloc_tex(texture::load(&display, "res/grass_nrm.jpg".to_string()));

        let basic_shader = res.alloc_shader(shader::load(&display, "res/basic"));

        let grass_material =
            Material::new(basic_shader, grass, grassnrm, [1.0, 1.0, 1.0], [3.0, 3.0]);
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

        GameState {
            bricks: bricks,
            grass: grass,
            text_size: 25.0,
        }
    }

    pub fn update(&mut self, context: &mut RenderContext, delta_time: f32) {
        context.camera.update(delta_time);
    }

    pub fn render_gui(&mut self, ui: &mut UIContext) {
        // copy the width and height variables of ui context
        let width = ui.win_width;
        let height = ui.win_height;

        let button_center_x = width / 2.0;
        let button_center_y = height / 2.0;

        if ui.render_button(self.bricks, button_center_x, button_center_y, 50.0, 50.0) {
            println!("Click");
        }

        ui.render_text(
            "Hello World!....",
            button_center_x,
            button_center_y,
            self.text_size,
            50.0,
        );
    }

    pub fn process_input(&mut self, context: &mut RenderContext, event: &glutin::WindowEvent) {
        context.camera.process_input(event);

        let input = match *event {
            glutin::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };

        let pressed = input.state == glutin::ElementState::Pressed;

        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };

        match key {
            glutin::VirtualKeyCode::V => self.text_size += 0.5,
            glutin::VirtualKeyCode::B => self.text_size -= 0.5,
            _ => (),
        };
    }

    pub fn process_input_device(
        &mut self,
        context: &mut RenderContext,
        event: &glutin::DeviceEvent,
    ) {
        context.camera.process_input_device(event);
    }
}
