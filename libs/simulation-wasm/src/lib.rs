extern crate wasm_logger;
extern crate web_sys;

use rand::prelude::ThreadRng;

mod timer;
use timer::Timer;

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
        // let _ = Timer::new("Simulation::new");
        wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let mut rng = thread_rng();
        let sim = sim::Simulation::random(&mut rng);
        Self { rng, sim }
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        JsValue::from_serde(&world).expect("failed to serialize world")
    }

    pub fn age(&self) -> usize {
        self.sim.age
    }

    pub fn step(&mut self) {
        // let _ = Timer::new("Simulation::step");
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
        let animal_position = animal.position();
        Self {
            x: animal_position.x,
            y: animal_position.y,
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
        let food_position = food.position();
        Self {
            x: food_position.x,
            y: food_position.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Simulation;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn stress_test() {
        let mut simulation = Simulation::new();
        (0..10_000).for_each(|_| simulation.step());
    }
}
