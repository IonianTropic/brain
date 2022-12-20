use crate::math;

const DEFAULT_WEIGHT: f64 = 1.0;

pub struct Node { // Neuron
    inputs: &'static Vec<Node>,
    weights: Vec<f64>,
}

impl Node {
    pub fn from_inputs(inputs: &'static Vec<Node>) -> Self {
        let mut weights = Vec::new();
        for _ in 0..inputs.len() {
            weights.push(DEFAULT_WEIGHT);
        }
        Self {
            inputs,
            weights,
        }
    }
    pub fn evaluate(&self) -> f64 {
        if self.inputs.is_empty() {
            // TODO: Return input value corresponding to this Node
            -1.0
        } else {
            let mut sum: f64;
            for i in 0..self.inputs.len() {
                sum += self.weights[i]*self.inputs[i].evaluate();
            }
            math::relu(sum)
        }
    }
}
