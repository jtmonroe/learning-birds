use rand::prelude::ThreadRng;

use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);

        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).unwrap()
    }

    pub fn step(&mut self) {
        self.sim.step(&mut self.rng)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

#[derive(Clone, Debug, Serialize)]
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
        Self {
            x: animal.position().x,
            y: animal.position().y,
            rotation: animal.rotation().angle(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::food::Food> for Food {
    fn from(food: &sim::food::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
