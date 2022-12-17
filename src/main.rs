use std::thread::current;

const DEFAULT_WEIGHT: f64 = 1.0;
const DEFAULT_VALUE: f64 = 1.0; // FIXME: what should this be?

fn main() {
    let network = create_network(vec![3,4,4,3]);
}

fn create_network(nodes_per_layer: Vec<usize>) -> Vec<&'static Node> {
    /*
    for i in 0..nodes_per_layer[j] {
        current_layer.push(Node::from_inputs(prev_layer));
    }
    prev_layer = current_layer
    current_layer = Vec::new()
    
    NEW
    // input case
    let mut prev_layer = ???
    // middle layer cases
    for node_count in nodes_per_layer { // Needs to iterate starting at nodes_per_layer[1] right?
        prev_layer = create_layer(prev_layer, node_count);
    }
    // output case
    tail/return prev_layer
     */

    // input case
    let mut prev_layer: Vec<&'static Node> = Vec::new();
    // middle case
    for node_count in nodes_per_layer {
        prev_layer = create_layer(prev_layer, node_count); // without a semicolon will this evaluate as a tail?
    }
    prev_layer
    // output case

}

fn create_layer(prev_layer: Vec<&'static Node>, node_count: usize) -> Vec<&'static Node> {
    let mut current_layer: Vec<&'static Node> = Vec::with_capacity(node_count);
    for i in 0..node_count {
        current_layer[i] = &Node::from_inputs(prev_layer);
    }
    current_layer
}

struct Node { // Neuron
    inputs: Vec<&'static Node>,
    weights: Vec<f64>,
    value: f64 // FIXME: what type should this be?
}

impl Node {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            weights: Vec::new(),
            value: DEFAULT_VALUE,
        }
    }
    fn from_inputs(inputs: Vec<&'static Node>) -> Self {
        let mut weights = Vec::new();
        let value = DEFAULT_VALUE;
        for i in 0..inputs.len() {
            weights.push(DEFAULT_WEIGHT);
        }
        Self {  
            inputs,
            weights,
            value,
        }
    }
    fn add_input_node(&mut self, node: &'static Node) {
        self.inputs.push(node);
        self.weights.push(DEFAULT_WEIGHT);
    }
    fn evaluate(&self) {
        
        // relu(sum(weights*inputs.evaluate()))
        let sum: f64;
        for i in 0..self.inputs.len() { //FIXME: make sure this looks good
            // weight[i]*inputs[i] or some shit
        }
    }
}

fn relu(x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        x
    }
}

fn back_prop(output_nodes: Vec<&'static Node>) {

}
