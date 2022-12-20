use crate::node::Node;

pub struct Network {
    pub input: Vec<Node>,
    pub output: Vec<Node>,
}

// Private
impl Network {
    fn create_layer(prev_layer: &'static Vec<Node>, node_count: usize) -> Vec<Node> {
        let mut current_layer: Vec<Node> = Vec::with_capacity(node_count);
        for i in 0..node_count {
            current_layer[i] = Node::from_inputs(prev_layer);
        }
        current_layer
    }
}

impl Network {
    pub fn new(nodes_per_layer: Vec<usize>) -> Self {
        let input: Vec<Node> = Vec::with_capacity(nodes_per_layer[0]);
        let mut current_layer = Self::create_layer(&input, nodes_per_layer[1]);
        for node_count in nodes_per_layer.iter().skip(2) {
            current_layer = Self::create_layer(&current_layer, *node_count);
        }
        let output = current_layer;
        Self {
            input,
            output,
        }
    }
}
