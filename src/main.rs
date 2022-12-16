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
    for node_count in nodes_per_layer {
        prev_layer = create_layer(prev_layer, node_count);
    }
    // output case
     */
}

fn create_layer(prev_layer: Vec<&'static Node>, node_count: usize) -> Vec<&'static Node> {
    let current_layer = Vec::with_capacity(node_count);
    
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
