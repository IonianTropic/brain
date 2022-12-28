mod math;

const DEFAULT_WEIGHT: f64 = 1.0;
const DEFAULT_VALUE: f64 = 0.0;

// TODO: evaluate() function
// TODO: implement accessible Edges for evaluate()

fn main() {
    let template = vec![2,3,1];

    let network = Network::new(template);

    println!("input indices: ");
    println!("{:?}", network.inputs);

    println!("output indices: ");
    println!("{:?}", network.outputs);

    println!("node values: ");
    println!("{:?}", network.nodes);
}


struct Network {
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Network {
    pub fn new(template: Vec<usize>) -> Self {
        // nodes
        let node_count = template.iter().sum();
        let mut nodes: Vec<Node> = Vec::new();
        for _ in 0..node_count {
            nodes.push(Node::new());
        }

        // inputs
        let mut inputs = Vec::new();
        for idx in 0..template[0] {
            inputs.push(idx);
        }

        // outputs
        let mut outputs = Vec::new();
        for idx in node_count - template[template.len()-1]..node_count {
            outputs.push(idx);
        }

        // layers
        let mut layer_map: Vec<Vec<usize>> = Vec::new();
        let mut node_idx: usize = 0;

        for layer_nodes in template.iter() {
            let mut current_layer = Vec::new();
            for _ in 0..*layer_nodes {
                current_layer.push(node_idx);
                node_idx += 1;
            }
            layer_map.push(current_layer);
        }

        // edging
        let mut edges: Vec<Edge> = Vec::new();

        let prev_nodes_iter = layer_map[0..template.len()-1].into_iter();
        let current_nodes_iter = layer_map[1..template.len()].into_iter();

        for (idx, (prev, current)) in prev_nodes_iter.zip(current_nodes_iter).enumerate() {
            for (i, j) in itertools::iproduct!(prev, current) {
                edges.push(Edge::new(*i, *j));
                nodes[*j].push_input(idx);
                nodes[*i].push_output(idx);
            }
        }

        Self {
            inputs,
            outputs,
            nodes,
            edges,
        }
    }
    fn set_inputs(&mut self, new_inputs: Vec<f64>) {
        for (idx, input) in self.inputs.iter().enumerate() {
            self.nodes[*input].value = new_inputs[idx];
        }
    }
    fn eval(&mut self) {
        // get output nodes
        // for each of those:
        //     relu(prev_nodes.sum())

        //prev_nodes = nodes[edges[current_node.inputs[i]].start]
    }
}

struct Edge {
    start: usize,
    end: usize,
    weight: f64,
}

impl Edge {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            weight: DEFAULT_WEIGHT,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: f64,
    inputs: Vec<usize>, // for edges: Vec<Edge>
    outputs: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            value: DEFAULT_VALUE,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    
    fn push_input(&mut self, idx: usize) {
        self.inputs.push(idx);
    }

    fn push_output(&mut self, idx: usize) {
        self.outputs.push(idx);
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_mut_value(&mut self) -> &mut f64 {
        &mut self.value
    }
}
