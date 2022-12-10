use crate::*;

#[derive(Debug)]
pub struct World {
    pub (crate) animals: Vec<Animal>,
    pub (crate) foods: Vec<Food>,
}

// TODO: implement better distribution
// TODO: Add parameters for the number of animals and foods
// TODO: 100% coverage
impl World {
    pub fn random(rng: &mut dyn RngCore, animals: usize, foods: usize) -> Self {
        let animals = (0..animals).map(|_| Animal::random(rng)).collect();

        let foods = (0..foods).map(|_| Food::random(rng)).collect();
        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

