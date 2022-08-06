use std::{fmt::Display, error::Error};
use rand::seq::SliceRandom;

use super::{Individual, RngCore};

#[derive(Debug)]
pub struct SelectionError {
    message: String
}

impl Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SelectionError: {}", self.message)
    }
}

impl Error for SelectionError {}

impl SelectionError {
    pub (crate) fn from_error(e: impl Error) -> Self {
        Self { message: format!("Selection failed to select due to error:\n {}", e) }
    }
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> Result<&'a I, SelectionError>
    where
        I: Individual;
}

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> Result<&'a I, SelectionError>
    where
        I: Individual,
    {
        match population.choose_weighted(rng, |individual| individual.fitness()) {
            Ok(individual) => Ok(individual),
            Err(e) => Err(SelectionError::from_error(e)),
        }
    }
}