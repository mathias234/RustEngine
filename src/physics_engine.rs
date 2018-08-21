extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;

use na::{Isometry3, Vector3};
use ncollide3d::shape::ShapeHandle;
use nphysics3d::object::{BodyHandle, Material};
use nphysics3d::volumetric::Volumetric;
use nphysics3d::world::World;
use quaternion;
use vector;

pub enum PhysicsShape {
    SphereShape,
    BoxShape,
}

pub struct PhysicsContext {
    world: World<f32>,
}

impl PhysicsContext {
    pub fn new() -> PhysicsContext {
        let mut world = World::new();
        world.set_gravity(Vector3::y() * -2.81);

        PhysicsContext { world: world }
    }

    pub fn step(&mut self) {
        self.world.step();
    }

    pub fn get_rigid_body_pos(
        &mut self,
        handle: &nphysics3d::object::BodyHandle,
    ) -> vector::Vector3 {
        let pos = (*self.world.rigid_body(*handle).unwrap()).position();
        vector::Vector3 {
            x: pos.translation.vector.x,
            y: pos.translation.vector.y,
            z: pos.translation.vector.z,
        }
    }

    pub fn get_rigid_body_rot(
        &mut self,
        handle: &nphysics3d::object::BodyHandle,
    ) -> quaternion::Quaternion {
        let rot = (*self.world.rigid_body(*handle).unwrap()).position();
        quaternion::Quaternion {
            x: rot.rotation.quaternion()[0],
            y: rot.rotation.quaternion()[1],
            z: rot.rotation.quaternion()[2],
            w: rot.rotation.quaternion()[3],
        }
    }

    pub fn add_rbody(
        &mut self,
        shape: ShapeHandle<f32>,
        position: vector::Vector3,
    ) -> nphysics3d::object::BodyHandle {
        let local_inertia = shape.inertia(1.0);
        let local_center_of_mass = shape.center_of_mass();

        let handle = self.world.add_rigid_body(
            Isometry3::new(Vector3::new(position.x, position.y, position.z), na::zero()),
            local_inertia,
            local_center_of_mass,
        );

        self.world.add_collider(
            0.01,
            shape.clone(),
            handle,
            Isometry3::identity(),
            Material::default(),
        );

        handle
    }

    pub fn add_collider(
        &mut self,
        shape: ShapeHandle<f32>,
        position: vector::Vector3,
    ) -> ncollide3d::world::CollisionObjectHandle {
        self.world.add_collider(
            0.1,
            shape.clone(),
            BodyHandle::ground(),
            Isometry3::new(Vector3::new(position.x, position.y, position.z), na::zero()),
            Material::default(),
        )
    }
}
