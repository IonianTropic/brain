use std::thread::current;

const DEFAULT_WEIGHT: f64 = 1.0;

fn main() {
    let network = create_network();
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
}

fn create_layer(prev_layer: Vec<&'static Node>, node_count: usize) -> Vec<&'static Node> {
    let current_layer: Vec<&'static Node> = Vec::with_capacity(node_count);
    for i in 0..node_count {
        current_layer[i] = &Node::from_inputs(prev_layer)
    }
    current_layer
}

fn create_input_layer(node_count: usize) -> Vec<&'static Node> {
    let current_layer: Vec<&'static Node> = Vec::with_capacity(node_count);
    for i in 0..node_count {
        current_layer[i] = &Node::new()
    }
    current_layer
}

struct Node { // Neuron
    inputs: Vec<&'static Node>,
    weights: Vec<f64>,
}

impl Node {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            weights: Vec::new(),
        }
    }
    fn new_input_node() -> Self {
        Self {
            // inputs: somehow just a value instead of Vec<Node>?
            //weights: just 1? idk my brain fried
        }
    }
    fn from_inputs(inputs: Vec<&'static Node>) -> Self {
        let weights = Vec::new();
        for i in 0..inputs.len() {
            weights.push(DEFAULT_WEIGHT);
        }
        Self {  
            inputs,
            weights,
        }
    }
    fn add_input_node(&mut self, node: &'static Node) {
        self.inputs.push(node);
        self.weights.push(DEFAULT_WEIGHT);
    }
    fn evaluate(&self) {
        // relu(sum(weights*inputs.evaluate()))
        // FIXME: evaluate has to be different on input nodes
        let sum: f64;
        for i in 0..self.inputs.len() { //FIXME: make sure this looks good

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
