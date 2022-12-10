use animal_individual::AnimalIndividual;
use log::info;
use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::FRAC_PI_2;
use std::ops::Div;

/// Delink the state changes from the underlying code.
/// Allow the Observers to mutate without hard linkages
pub trait Observer<T: Clone> {
    fn set(&mut self, new_value: T) -> bool;
    fn get(&self) -> T;
}

pub mod animal;
mod animal_individual;
mod brain;
pub mod eye;
pub mod food;
pub mod world;

use ga::{
    crossover_method::UniformCrossover, mutation_method::GaussianMutation,
    selection_method::RouletteWheelSelection,
};
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use {animal::*, brain::*, eye::*, food::*, world::*};

const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 1000;

pub struct Simulation {
    world: World,
    genetic_algorithm:
        ga::GeneticAlgorithm<RouletteWheelSelection, UniformCrossover, GaussianMutation>,
    pub age: usize,
    pub generation_length: usize,
}

const EPSILON: f32 = 0.01;

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, animals: usize, foods: usize) -> Self {
        info!("new random simulation");
        let ga = ga::GeneticAlgorithm::new(
            RouletteWheelSelection::default(),
            UniformCrossover::default(),
            GaussianMutation::new(0.01, 0.3),
        );

        Self {
            world: World::random(rng, animals, foods),
            genetic_algorithm: ga,
            age: 0,
            generation_length: GENERATION_LENGTH,
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    // TODO: Kill animals
    // TODO: Mate birds
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.age += 1;

        if self.age > self.generation_length {
            info!("Old Generation aging out. New Generation Evolving.");
            self.evolve(rng);
        }
    }

    pub fn average_fitness(&self) -> f32 {
        self.world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .map(|it| it.fitness)
            .sum::<f32>()
            .div(self.world.animals.len() as f32)
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        info!("stepping forward a generation");
        self.age = 0;

        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let evolved_population = self.genetic_algorithm.evolve(rng, &current_population);

        self.world.animals = evolved_population
            .into_iter()
            .map(|animal_individual| animal_individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);
            let response = animal.see(vision);

            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(animal.speed, 0.0);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance < EPSILON {
                    food.position = rng.gen();
                    animal.satiation += 1
                }
            }
        }
    }
}
