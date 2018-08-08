extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;

use na::{Isometry3, Point3, Vector3};
use ncollide3d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics3d::object::{BodyHandle, Material};
use nphysics3d::volumetric::Volumetric;
use nphysics3d::world::World;
use vector;

pub struct PhysicsContext {
    world: World<f32>,
}

impl PhysicsContext {
    pub fn new() -> PhysicsContext {
        let mut world = World::new();
        world.set_gravity(Vector3::y() * -9.81);

        PhysicsContext { world: world }
    }

    pub fn step(&mut self) {
        self.world.step();
    }

    pub fn get_rigid_body_pos(
        &mut self,
        handle: &nphysics3d::object::BodyHandle,
    ) -> vector::Vector3 {
        let mut pos = (*self.world.rigid_body(*handle).unwrap()).position();
        vector::Vector3 {
            x: pos.translation.vector.x,
            y: pos.translation.vector.y,
            z: pos.translation.vector.z,
        }
    }

    pub fn add_cube_rigid_body(&mut self, size: vector::Vector3) -> nphysics3d::object::BodyHandle {
        let cuboid = ShapeHandle::new(Cuboid::new(Vector3::new(size.x, size.y, size.z)));
        let local_inertia = cuboid.inertia(0.0);
        let local_center_of_mass = cuboid.center_of_mass();

        self.world.add_rigid_body(
            Isometry3::new(Vector3::x() * 2.0, na::zero()),
            local_inertia,
            local_center_of_mass,
        )
    }

    pub fn add_cube_collider(
        &mut self,
        size: vector::Vector3,
    ) -> ncollide3d::world::CollisionObjectHandle {
        let cuboid = ShapeHandle::new(Cuboid::new(Vector3::new(size.x, size.y, size.z)));

        self.world.add_collider(
            0.04,
            cuboid,
            BodyHandle::ground(),
            Isometry3::identity(),
            Material::default(),
        )
    }
}
