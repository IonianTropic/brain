use crate::node::Node;

pub struct Network<'a> {
    pub input: &'a Vec<Node<'a>>,
    pub output: Vec<Node<'a>>,
}

// Private
impl Network<'_> {
    //Creates a vec of nodes that all contain the previous layer as an input
    //Needs references to prev_layer and the number of nodes to create
    fn create_layer<'a>(prev_layer: &'a  Vec<Node<'a>>, node_count: usize) -> Vec<Node<'a>> { 
        let mut layer_nodes: Vec<Node> = Vec::with_capacity(node_count);
        
        for i in 0..node_count {
            layer_nodes[i] = Node::from_inputs(&prev_layer);
        }
        layer_nodes
    }
}

impl <'a> Network<'a> {
    pub fn new(input: &'a Vec<Node<'a>>, network_template: Vec<usize>) -> Self {
        //creating empty input vec. To be filled in later


        let mut mid_layer: Vec<Node<'a>> = Self::create_layer(&input, network_template[1]);

        for node_count in network_template.iter().skip(2) {
            // let layer_ref: &'a Vec<Node> = &current_layer;
            let prev_mid_layer = mid_layer;
            mid_layer = Self::create_layer(&prev_mid_layer, *node_count);
        }

        let output = mid_layer;

        Self {
            input,
            output,
        }
    }
}