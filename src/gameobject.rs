extern crate glium;
extern crate ncollide3d;
extern crate nphysics3d;

use math_helper;
use na::{Isometry3, Point3};
use ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle, Material};
use physics_engine::PhysicsContext;
use quaternion::Quaternion;
use vector::Vector3;

pub struct GameObject {
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub model: usize,
    pub shader_program: usize,
    pub texture: usize,
    pub normal_map: usize,

    // Physics variables
    pub physics_enabled: bool,
    pub rigid_body_handle: Option<nphysics3d::object::BodyHandle>,
    pub collision_handle: Option<ncollide3d::world::CollisionObjectHandle>,
}

impl GameObject {
    pub fn new(
        physics: &mut PhysicsContext,
        name: String,
        position: Vector3,
        rotation: Quaternion,
        model: usize,
        shader_program: usize,
        texture: usize,
        normal_map: usize,
        physics_enabled: bool,
    ) -> GameObject {
        let mut rbody_handle: Option<nphysics3d::object::BodyHandle> = None;
        let mut collision_handle: Option<ncollide3d::world::CollisionObjectHandle> = None;

        let pos = position;

        if physics_enabled {
            println!("Physics enabled");
            rbody_handle = Some(physics.add_cube_rigid_body(pos, Vector3::new(5.0, 1.0, 5.0)));
        } else {
            collision_handle =
                Some(physics.add_cube_collider(pos, Vector3::new(500.0, 0.1, 500.0)));
        }

        GameObject {
            rigid_body_handle: rbody_handle,
            collision_handle: collision_handle,
            name: name,
            position: position,
            rotation: rotation,
            model: model,
            shader_program: shader_program,
            texture: texture,
            normal_map: normal_map,
            physics_enabled: physics_enabled,
        }
    }

    pub fn update(&mut self, physics: &mut PhysicsContext) {
        if self.physics_enabled {
            let mut handle = self.rigid_body_handle.unwrap();

            self.position = physics.get_rigid_body_pos(&handle);
            self.rotation = physics.get_rigid_body_rot(&handle);
            println!(
                "Position: [{}, {}, {}]",
                self.position.x, self.position.y, self.position.z
            );

            self.rigid_body_handle = Some(handle);
        }
    }

    pub fn rotate(&mut self, axis: Vector3, angle: f32) {
        let rot = Quaternion::new_axis_angle(axis, angle);

        let old_rot = self.rotation;

        self.rotation = rot.mul_quat(old_rot);
    }

    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        self.rotation.to_rotation_matrix()
    }

    fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position.x, self.position.y, self.position.z, 1.0],
        ]
    }

    pub fn get_model_matrix(&self) -> [[f32; 4]; 4] {
        math_helper::mat_mul(self.get_rotation_matrix(), self.get_translation_matrix())
    }
}
