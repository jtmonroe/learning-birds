use rand::{thread_rng, Rng, RngCore};
use std::ops::Add;

// TODO: Upgrade to matrices
pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(layers: &Vec<LayerTopology>) -> Self {
        assert!(layers.len() > 1);
        let mut rng = thread_rng();
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons, &mut rng))
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(&inputs))
    }
}

impl Layer {
    fn random(input_neurons: usize, output_neurons: usize, rng: &mut dyn RngCore) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons, rng))
            .collect();
        Self { neurons }
    }

    fn propagate(&self, inputs: &Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    fn random(neurons: usize, rng: &mut dyn RngCore) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..neurons).map(|_| rng.gen_range(-1.0..1.0)).collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &Vec<f32>) -> f32 {
        assert_eq!(self.weights.len(), inputs.len());

        inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>()
            .add(self.bias)
            .max(0.0)
    }
}
