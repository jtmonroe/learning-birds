extern crate wasm_logger;
extern crate web_sys;

use rand::prelude::ThreadRng;

pub mod timer;

pub mod observer;
use observer::*;

use lib_simulation as sim;
use rand::prelude::*;

pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

impl Simulation {
    pub fn new(animals: usize, foods: usize) -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng, animals, foods);
        Self { rng, sim }
    }

    pub fn raw_world(&self) -> World {
        self.sim.world().into()
    }

    pub fn age(&self) -> usize {
        self.sim.age
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng)
    }
}

#[derive(Clone, Debug)]
pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

impl From<&sim::world::World> for World {
    fn from(world: &sim::world::World) -> Self {
        let animals = world.animals().iter().map(Animal::from).collect();
        let food = world.foods().iter().map(Food::from).collect();

        Self { animals, food }
    }
}

impl From<&sim::animal::Animal> for Animal {
    fn from(animal: &sim::animal::Animal) -> Self {
        let animal_position = animal.position();
        Self {
            x: animal_position.x,
            y: animal_position.y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::food::Food> for Food {
    fn from(food: &sim::food::Food) -> Self {
        let food_position = food.position();
        Self {
            x: food_position.x,
            y: food_position.y,
        }
    }
}
