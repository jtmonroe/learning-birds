use nalgebra as na;
use rand::{Rng, RngCore};
pub mod animal;
pub mod food;
pub mod world;
pub mod eye;

use {animal::*, food::*, world::*, eye::*};

pub struct Simulation {
    world: World,
}

const EPSILON: f32 = 0.01;

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    // TODO: Kill animals
    // TODO: Mate birds
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_movements();
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance < EPSILON {
                    food.position = rng.gen();
                }
            }
        }
    }
}



