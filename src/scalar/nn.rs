use rand::random_range;

use crate::{scalar::Value};

pub struct Neuron {
    w: Vec<Value>,
    b: Value,
}

impl Neuron {

    fn new(n_inputs: usize) -> Self {
        Neuron {
            w: (0..n_inputs).map(|_| Value::new(random_range(-1.0..1.0))).collect(),
            b: Value::new(random_range(-1.0..1.0)),
            
        }
    }

    fn forward(&self, x: &[Value]) -> Value {
        let mut sum = self.b;
        for (wi, xi) in self.w.iter().zip(x.iter()) {
            sum = sum + *wi * *xi;
        }
        sum.tanh()
    }

    fn parameters(&self) -> Vec<Value> {
        let mut p = self.w.clone();
        p.push(self.b);
        p
    }
}

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(n_inputs: usize, n_outputs: usize) -> Layer {
        Layer {
            neurons: (0..n_outputs).map(|_| Neuron::new(n_inputs)).collect(),
    }

}

    fn forward(&self, x: &Vec<Value>) -> Vec<Value> {
        self.neurons.iter().map(|n| n.forward(&x)).collect()
    }

    fn parameters(&self) -> Vec<Value> {
        self.neurons.iter().flat_map(|n| n.parameters()).collect()
    }
}

pub struct Mlp {
    layers: Vec<Layer>,
}

impl Mlp {
    pub fn new(sizes: &[usize]) -> Self {
        Mlp {
            layers: sizes.windows(2).map(|p| Layer::new(p[0], p[1])).collect(),
        }
    }

    pub fn forward(&self, x: Vec<Value>) -> Vec<Value>{

        let mut curr = x;

        for layer in &self.layers {
            curr = layer.forward(&curr);
        }
        
        curr

    }

    pub fn parameters(&self) -> Vec<Value> {
        self.layers.iter().flat_map(|l| l.parameters()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mlp_shapes() {
        let model = Mlp::new(&[2, 4, 1]);
        assert_eq!(model.parameters().len(), 17);

        let output = model.forward(vec![Value::new(0.5), Value::new(-1.0)]);
        assert_eq!(output.len(), 1);
        assert!(output[0].data() > -1.0 && output[0].data() < 1.0);
    }
}