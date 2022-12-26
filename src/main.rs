mod math;

const DEFAULT_WEIGHT: f64 = 1.0;
const DEFAULT_VALUE: f64 = 0.0;

//TODO: evaluate() function
//TODO: implement accessible Edges for evaluate()

//TODO: implement modify inputs/get outputs function to get the values of stored indices
fn main() {
    let template = vec![2,3,1];

    let network = Network::new(template);

    println!("input indices: ");
    vprint(&network.inputs);

    println!("output indices: ");
    vprint(&network.outputs);

    println!("node values: ");
    vprint(&network.nodes);
}

fn vprint<T: std::fmt::Display> (vec: &Vec<T>) {
    for i in vec {
        print!("{} ", i);
    }
    println!();
}

struct Network {
    inputs: Vec<usize>,
    outputs: Vec<usize>,
    nodes: Vec<f64>,
    edges: Vec<Edge>,
}

impl Network {
    fn new(template: Vec<usize>) -> Self {
        // nodes
        let node_count = template.iter().sum();
        let mut nodes: Vec<f64> = Vec::new();
        for _ in 0..node_count {
            nodes.push(DEFAULT_VALUE);
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
        // EX with template <2,3,1>
        // <<0,1>,<2,3,4>,<5>>
        let mut layer_map: Vec<Vec<usize>> = Vec::new();
        let mut node_idx: usize = 0;

        for layer_nodes in template[0..].into_iter() {
            let mut current_layer = Vec::new();
            for _ in 0..*layer_nodes {
                current_layer.push(node_idx);
                node_idx += 1;
            }
            layer_map.push(current_layer);
        }

        // edging
        let mut edges: Vec<Edge> = Vec::new();

        let prev_nodes_iter = layer_map[0..template.len()-1].into_iter(); //zero cost abstraction right?
        let current_nodes_iter = layer_map[1..template.len()].into_iter();

        // iterates over each adjacent pair of node layers and then the product of those two layers
        for (prev, current) in prev_nodes_iter.zip(current_nodes_iter) {
            for (i, j) in itertools::iproduct!(prev, current) {
                edges.push(Edge::new(*i, *j)); // Is it fine to dereference here?
            }
        }

        Self {
            inputs,
            outputs,
            nodes,
            edges,
        }
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
