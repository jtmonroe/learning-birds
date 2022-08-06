extern crate nalgebra;

use rand::RngCore;

pub mod layer;
use layer::Layer;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn random(mut rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons, &mut rng))
            .collect();

        Self { layers }
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            let count = weights.count();
            panic!(
                "Network tried to take too many weights. {} items remaining",
                count
            );
        }

        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers.iter().flat_map(|layer| layer.weights())
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let vector = nalgebra::DVector::from_vec(inputs);
        self.layers
            .iter()
            .fold(vector, |inputs, layer| layer.propagate(&inputs))
            .data
            .as_vec()
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::{LayerTopology, Network};
    use nalgebra::{dvector, matrix, vector, Vector};
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_random_lifecycle() {
        let layers = &[
            LayerTopology { neurons: 3 },
            LayerTopology { neurons: 4 },
            LayerTopology { neurons: 1 },
        ];
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let network = Network::random(&mut rng, layers);
        let second_network = Network::from_weights(layers, &mut network.weights());

        assert_eq!(network.layers, second_network.layers);
    }

    #[test]
    fn test_propagate() {
        let actual_weights: nalgebra::Matrix<f32, _, _, _> = matrix![2.0, 3.0; 4.0, 5.0; 6.0, 7.0];
        let actual_biases: nalgebra::Vector<f32, _, _> = vector![1.0, 1.0, 1.0];
        let input_vector = dvector![1.0, 1.0];
        let result: Vector<f32, _, _> = actual_weights * &input_vector + actual_biases;

        let (output_neurons, input_neurons) = (actual_weights.nrows(), actual_weights.ncols());

        let neuron_weights = vec![1.0, 2.0, 3.0, 1.0, 4.0, 5.0, 1.0, 6.0, 7.0].into_iter();
        let layers = &[
            LayerTopology {
                neurons: input_neurons,
            },
            LayerTopology {
                neurons: output_neurons,
            },
        ];

        let network = Network::from_weights(layers, neuron_weights);

        let expected_result = result.into_iter().cloned().collect::<Vec<f32>>();
        let actual_result = network.propagate(input_vector.data.as_vec().to_owned());
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    #[should_panic]
    fn test_from_weight_panic() {
        let actual_weights: nalgebra::Matrix<f32, _, _, _> = matrix![2.0, 3.0; 4.0, 5.0; 6.0, 7.0];
        let (output_neurons, input_neurons) = (actual_weights.nrows(), actual_weights.ncols());

        let neuron_weights =
            vec![1.0, 2.0, 3.0, 1.0, 4.0, 5.0, 1.0, 6.0, 7.0, 100000000.0].into_iter();
        let layers = &[
            LayerTopology {
                neurons: input_neurons,
            },
            LayerTopology {
                neurons: output_neurons,
            },
        ];
        Network::from_weights(layers, neuron_weights);
    }
}
