use rapier::dynamics::{JointSet, RigidBodyBuilder, RigidBodySet};
use rapier::geometry::ColliderSet;
use rapier::math::Vector;
use rapier::world::World as RWorld;
use wasm_bindgen::prelude::*;

use crate::dynamic::{Joint, JointDesc, RigidBody, RigidBodyDesc};
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct World {
    world: RWorld,
    bodies: Rc<RefCell<RigidBodySet>>,
    colliders: Rc<RefCell<ColliderSet>>,
    joints: Rc<RefCell<JointSet>>,
}

#[wasm_bindgen]
impl World {
    #[cfg(feature = "dim2")]
    #[wasm_bindgen(constructor)]
    pub fn new(gravity_x: f32, gravity_y: f32) -> Self {
        let world = RWorld::new(Vector::new(gravity_x, gravity_y));
        Self {
            world,
            bodies: Rc::new(RefCell::new(RigidBodySet::new())),
            colliders: Rc::new(RefCell::new(ColliderSet::new())),
            joints: Rc::new(RefCell::new(JointSet::new())),
        }
    }

    #[cfg(feature = "dim3")]
    #[wasm_bindgen(constructor)]
    pub fn new(gravity_x: f32, gravity_y: f32, gravity_z: f32) -> World {
        let world = RWorld::new(Vector::new(gravity_x, gravity_y, gravity_z));
        Self {
            world,
            bodies: Rc::new(RefCell::new(RigidBodySet::new())),
            colliders: Rc::new(RefCell::new(ColliderSet::new())),
            joints: Rc::new(RefCell::new(JointSet::new())),
        }
    }

    pub fn step(&mut self) {
        self.world.step(
            &mut *self.bodies.borrow_mut(),
            &mut *self.colliders.borrow_mut(),
            &mut *self.joints.borrow_mut(),
            &(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn timestep(&self) -> f32 {
        self.world.integration_parameters.dt()
    }

    #[wasm_bindgen(setter)]
    pub fn set_timestep(&mut self, timestep: f32) {
        self.world.integration_parameters.set_dt(timestep)
    }

    #[wasm_bindgen(getter)]
    pub fn max_velocity_iterations(&self) -> usize {
        self.world.integration_parameters.max_velocity_iterations
    }

    #[wasm_bindgen(getter)]
    pub fn max_position_iterations(&self) -> usize {
        self.world.integration_parameters.max_position_iterations
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_velocity_iterations(&mut self, niter: usize) {
        self.world.integration_parameters.max_velocity_iterations = niter
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_position_iterations(&mut self, niter: usize) {
        self.world.integration_parameters.max_position_iterations = niter
    }

    pub fn create_rigid_body(&mut self, body: &RigidBodyDesc) -> RigidBody {
        let builder: RigidBodyBuilder = (*body).into();
        let rb = builder.build();
        let bodies = self.bodies.clone();
        let colliders = self.colliders.clone();
        let handle = bodies.borrow_mut().insert(rb);
        RigidBody {
            bodies,
            colliders,
            handle,
        }
    }

    pub fn create_joint(
        &mut self,
        joint: &JointDesc,
        parent1: &RigidBody,
        parent2: &RigidBody,
    ) -> Joint {
        let mut bodies = self.bodies.borrow_mut();
        let handle =
            self.joints
                .borrow_mut()
                .insert(&mut *bodies, parent1.handle, parent2.handle, joint.0);

        Joint {
            bodies: self.bodies.clone(),
            joints: self.joints.clone(),
            handle,
        }
    }
}