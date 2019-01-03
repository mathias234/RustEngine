extern crate glium;
extern crate ncollide3d;
extern crate nphysics3d;

use material::*;
use math_helper;
use na::Vector3 as PhysicsVec3;
use ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use physics_engine;
use physics_engine::PhysicsContext;
use quaternion::Quaternion;
use resource_manager::*;
use vector::Vector3;

pub struct GameObject {
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub model: Resource,
    pub bounding_box: [f32; 3],
    pub material: Material,
    pub overlay: bool,

    // Physics variables
    pub physics_enabled: bool,
    pub rigid_body_handle: Option<nphysics3d::object::BodyHandle>,
    pub collision_handle: Option<ncollide3d::world::CollisionObjectHandle>,
}

#[allow(dead_code)]
impl GameObject {
    pub fn new(
        resources: &mut ResourceContext,
        name: String,
        position: Vector3,
        rotation: Quaternion,
        model: Resource,
        material: Material,
    ) -> GameObject {
        let bounding_box = resources.get_model_ref(model).bounding_box;

        GameObject {
            rigid_body_handle: None,
            collision_handle: None,
            name: name,
            position: position,
            rotation: rotation,
            model: model,
            material: material,
            physics_enabled: false,
            bounding_box: bounding_box,
            overlay: false,
        }
    }

    pub fn update(&mut self, physics: &mut PhysicsContext) {
        if self.physics_enabled && self.rigid_body_handle.is_some() {
            let handle = self.rigid_body_handle.unwrap();

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

        if shape.is_some() {
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

        if shape.is_some() {
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
