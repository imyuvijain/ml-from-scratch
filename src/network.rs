use rand::{self, random_range};
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    weights : Vec<Vec<f32>>,
    biases : Vec<f32>,
}

impl Layer {
    pub fn create(size: usize, prev: usize) -> Self {
        
        let weights: Vec<Vec<f32>> = (0..size)
            .map(|_| (0..prev)
                .map(|_| random_range(-0.5..0.5))
                .collect())
            .collect();

        let biases: Vec<f32> = (0..size)
            .map(|_| random_range(-0.5..0.5))
            .collect();

        Layer{weights : weights, biases : biases}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn calculate(&self, input: &Vec<f32>) -> Vec<f32> {

        let mut values: Vec<f32> = input.clone();

        for layer in &self.layers {
            values = Self::dot(&values, layer);
        }

        values.iter().map(|x| x.tanh()).collect()

    }

    fn dot(inputs: &Vec<f32>, layer: &Layer) -> Vec<f32> {

        let size = layer.biases.len();

        let mut outputs: Vec<f32> = vec!(0.0 ; size);

        for node in 0..size {

            let mut sum: f32 = 0.0;

            for (index, input) in inputs.iter().enumerate() {
                sum += layer.weights[node][index] * input; 
            }
            outputs[node] += sum+layer.biases[node];

        }
        outputs
    
}

    pub fn create(sizes: &[usize]) -> Network {
        let layers: Vec<Layer> = sizes
            .windows(2)
            .map(|w| Layer::create(w[1], w[0]))
            .collect();

        Network { layers }
    }

    pub fn mutate(&mut self) {
    for layer in &mut self.layers {
        for neuron_weights in &mut layer.weights {
            for weight in neuron_weights {
                if random_range(0.0..1.0) < 0.8 {
                    *weight += random_range(-0.05..0.05);
                }
            }
        }
        for bias in &mut layer.biases {
            if random_range(0.0..1.0) < 0.8 {
                *bias += random_range(-0.1..0.1);
            }
        }
    }
}

}
