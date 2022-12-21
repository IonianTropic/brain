use crate::math;

const DEFAULT_WEIGHT: f64 = 1.0;

#[derive(Clone)]
pub struct Node<'a> { // Neuron
    inputs: &'a Vec<Node<'a>>,
    weights: Vec<f64>,
}

impl <'a> Node<'a> {
    pub fn from_inputs(inputs: &'a Vec<Node>) -> Self {
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
            let mut sum: f64 = 0.0;
            for i in 0..self.inputs.len() {
                sum += self.weights[i]*self.inputs[i].evaluate();
            }
            math::relu(sum)
        }
    }
}
