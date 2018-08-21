extern crate glium;
extern crate ncollide3d;
extern crate nphysics3d;

use math_helper;
use na::Vector3 as PhysicsVec3;
use na::{Isometry3, Point3};
use ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle, Material};
use nphysics3d::volumetric::Volumetric;
use physics_engine;
use physics_engine::PhysicsContext;
use quaternion::Quaternion;
use resource_manager::ResourceContext;
use vector::Vector3;

pub struct GameObject {
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub model: usize,
    pub bounding_box: [f32; 3],
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
        resources: &mut ResourceContext,
        name: String,
        position: Vector3,
        rotation: Quaternion,
        model: usize,
        shader_program: usize,
        texture: usize,
        normal_map: usize,
    ) -> GameObject {
        let bounding_box = resources.get_model_ref(model).bounding_box;

        GameObject {
            rigid_body_handle: None,
            collision_handle: None,
            name: name,
            position: position,
            rotation: rotation,
            model: model,
            shader_program: shader_program,
            texture: texture,
            normal_map: normal_map,
            physics_enabled: false,
            bounding_box: bounding_box,
        }
    }

    pub fn update(&mut self, physics: &mut PhysicsContext) {
        if self.physics_enabled {
            let mut handle = self.rigid_body_handle.unwrap();

            self.position = physics.get_rigid_body_pos(&handle);
            self.rotation = physics.get_rigid_body_rot(&handle);

            self.rigid_body_handle = Some(handle);
        }
    }

    fn get_shape(&self, physics_shape: physics_engine::PhysicsShape) -> Option<ShapeHandle<f32>> {
        let shape: Option<ShapeHandle<f32>>;

        match physics_shape {
            physics_engine::PhysicsShape::BoxShape => {
                shape = Some(ShapeHandle::new(Cuboid::new(PhysicsVec3::new(
                    self.bounding_box[0] / 2.0,
                    self.bounding_box[1] / 2.0,
                    self.bounding_box[2] / 2.0,
                ))));
            }
            physics_engine::PhysicsShape::SphereShape => {
                shape = Some(ShapeHandle::new(Ball::new(self.bounding_box[0] / 2.0)));
            }
        }

        shape
    }

    pub fn add_collider(
        mut self,
        physics_context: &mut PhysicsContext,
        physics_shape: physics_engine::PhysicsShape,
    ) -> Self {
        let shape = self.get_shape(physics_shape);

        if (shape.is_some()) {
            self.collision_handle =
                Some(physics_context.add_collider(shape.unwrap(), self.position));
        } else {
            panic!("Unable to find a shape for physics object")
        }
        self
    }

    pub fn add_rigidbody(
        mut self,
        physics_context: &mut PhysicsContext,
        physics_shape: physics_engine::PhysicsShape,
    ) -> Self {
        let shape = self.get_shape(physics_shape);

        if (shape.is_some()) {
            self.rigid_body_handle = Some(physics_context.add_rbody(shape.unwrap(), self.position));
            self.physics_enabled = true;
        } else {
            panic!("Unable to find a shape for physics object")
        }

        self
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
