mod math;

const DEFAULT_WEIGHT: f64 = 1.0;
const DEFAULT_VALUE: f64 = 0.0;

fn main() {
    let mut node_list: Vec<f64> = Vec::new();
    let mut edge_list: Vec<Edge> = Vec::new();
    // network template: <2,3,1>

    // make input node(s) (0,1)
    node_list.push(DEFAULT_VALUE);
    node_list.push(DEFAULT_VALUE);
    // make middle layer node(s) (2,3,4)
    node_list.push(DEFAULT_VALUE);
    node_list.push(DEFAULT_VALUE);
    node_list.push(DEFAULT_VALUE);
    //make output node(s) (5)
    node_list.push(DEFAULT_VALUE);

    // connect inputs to middle
    edge_list.push(Edge::new(0, 2));
    edge_list.push(Edge::new(0, 3));
    edge_list.push(Edge::new(0, 4));
    edge_list.push(Edge::new(1, 2));
    edge_list.push(Edge::new(1, 3));
    edge_list.push(Edge::new(1, 4));
    // connect middle to middle
    // connect middle to output
    edge_list.push(Edge::new(2, 5));
    edge_list.push(Edge::new(3, 5));
    edge_list.push(Edge::new(4, 5));
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
        let inputs = Vec::new();
        for idx in 0..template[0] {
            inputs.push(idx);
        }
        // outputs
        let outputs = Vec::new();
        for idx in node_count - template[template.len()-1]..node_count {
            outputs.push(idx);
        }
        // edging
        let mut edges: Vec<Edge> = Vec::new();
        // for (prev, current) in template[0..template.len()-1].into_iter().zip(template[1..template.len()].into_iter()) {
        //     for i in 
        // }


        Self {
            inputs,
            outputs,
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
