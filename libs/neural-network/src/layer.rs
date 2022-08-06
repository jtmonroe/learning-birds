use log::debug;
use nalgebra::{Const, Dynamic, OMatrix, OVector, VecStorage};
use rand::{Rng, RngCore};

const ONE: Const<1> = Const::<1>;

#[derive(Debug, PartialEq)]
pub(crate) struct Layer {
    weights: nalgebra::OMatrix<f32, Dynamic, Dynamic>,
    biases: nalgebra::OVector<f32, Dynamic>,
}

impl Layer {
    pub(crate) fn random(
        input_neurons: usize,
        output_neurons: usize,
        rng: &mut dyn RngCore,
    ) -> Self {
        debug!(
            "create new random layer with weights dim ({}, {}) and bias dim ({})",
            output_neurons, input_neurons, output_neurons
        );
        let ncols = Dynamic::new(input_neurons);
        let nrows = Dynamic::new(output_neurons);

        let weights = OMatrix::from_fn_generic(nrows, ncols, |_, _| rng.gen_range(-1.0..1.0));
        let biases = OVector::from_fn_generic(nrows, ONE, |_, _| rng.gen_range(-1.0..=1.0));

        Self { weights, biases }
    }

    pub(crate) fn propagate(
        &self,
        inputs: &nalgebra::Vector<f32, Dynamic, VecStorage<f32, nalgebra::Dynamic, Const<1>>>,
    ) -> nalgebra::Vector<f32, Dynamic, VecStorage<f32, nalgebra::Dynamic, Const<1>>> {
        (&self.weights * inputs + &self.biases).map(|x| x.max(0.0))
    }

    pub(crate) fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        use std::iter::once;

        self.weights
            .row_iter()
            .zip(self.biases.into_iter())
            .flat_map(|(row, bias)| {
                once(bias)
                    .chain(row.into_iter())
                    .map(|x| x.to_owned())
                    .collect::<Vec<_>>()
            })
    }

    pub(crate) fn from_weights(
        input_neurons: usize,
        output_neurons: usize,
        neuron_weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        debug!(
            "create new layer from weights dim ({}, {}) and bias dim ({})",
            output_neurons, input_neurons, output_neurons
        );
        let ncols = Dynamic::new(input_neurons);
        let nrows = Dynamic::new(output_neurons);

        let mut weights = OMatrix::from_element_generic(nrows, ncols, 0.0);
        let mut biases = OVector::from_element_generic(nrows, ONE, 0.0);

        for row in 0..output_neurons {
            biases[(row)] = neuron_weights.next().expect("failed to receive bias");
            for col in 0..input_neurons {
                debug!("adding weight from iterator at pos ({}, {})", row, col);
                weights[(row, col)] = neuron_weights
                    .next()
                    .expect(&format!("failed to receive weight ({}, {})", row, col));
            }
        }

        Self { weights, biases }
    }
}

#[cfg(test)]
mod tests {
    use super::Layer;
    use nalgebra::{dvector, matrix, vector};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test_log::test]
    fn random_lifecycle_test() {
        let (input_neurons, output_neurons) = (10, 10);
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let layer = Layer::random(input_neurons, output_neurons, &mut rng);
        let second_layer = Layer::from_weights(input_neurons, output_neurons, &mut layer.weights());

        assert_eq!(layer.weights, second_layer.weights);
        assert_eq!(layer.biases, second_layer.biases);
    }

    #[test_log::test]
    fn propagate_test() {
        let actual_weights: nalgebra::Matrix<f32, _, _, _> = matrix![2.0, 3.0; 4.0, 5.0; 6.0, 7.0];
        let actual_biases: nalgebra::Vector<f32, _, _> = vector![1.0, 1.0, 1.0];

        let (output_neurons, input_neurons) = (actual_weights.nrows(), actual_weights.ncols());

        let mut neuron_weights = vec![1.0, 2.0, 3.0, 1.0, 4.0, 5.0, 1.0, 6.0, 7.0].into_iter();
        let layer = Layer::from_weights(input_neurons, output_neurons, &mut neuron_weights);

        assert_eq!(actual_weights, layer.weights);
        assert_eq!(actual_biases, layer.biases);

        let input_vector = dvector![1.0, 1.0];
        assert_eq!(
            actual_weights * &input_vector + actual_biases,
            layer.propagate(&input_vector)
        );
    }
}
