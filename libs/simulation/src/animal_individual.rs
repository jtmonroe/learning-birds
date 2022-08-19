use crate::*;

pub struct AnimalIndividual {
    pub fitness: f32,
    chromosome: ga::Chromosome,
}

impl ga::Individual for AnimalIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn from_chromosome(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self { fitness: animal.satiation as f32, chromosome: animal.as_chromosome() }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}
