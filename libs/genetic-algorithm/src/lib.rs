#![feature(type_alias_impl_trait)]

pub mod selection_method;
use log::warn;
use mutation_method::MutationMethod;
use selection_method::SelectionMethod;

pub mod crossover_method;
use crossover_method::CrossoverMethod;

pub mod mutation_method;

use rand::{Rng, RngCore};
use std::ops::Index;

pub struct GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    selection_method: S,
    crossover_method: C,
    mutation_method: M,
}

impl<S, C, M> GeneticAlgorithm<S, C, M>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    M: MutationMethod,
{
    pub fn new(selection_method: S, crossover_method: C, mutation_method: M) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        (0..population.len())
            .map(|_| {
                let parent_a = match self.selection_method.select(rng, population) {
                    Ok(i) => i.chromosome(),
                    Err(e) => {
                        warn!("{}", e);
                        population[0].chromosome()
                    }
                };
                let parent_b = match self.selection_method.select(rng, population) {
                    Ok(i) => i.chromosome(),
                    Err(e) => {
                        warn!("{}", e);
                        population[1].chromosome()
                    }
                };

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);
                I::from_chromosome(child)
            })
            .collect()
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn from_chromosome(chromosome: Chromosome) -> Self;
}

pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = impl Iterator<Item = f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
