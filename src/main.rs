mod math;
mod node;
mod Network;

use Network::Network as Net;


fn main() {
    let network_template = vec![3,4,4,3];
    let mut input: Vec<node::Node> = Vec::with_capacity(network_template[0]);
    let network = Net::new(&input, network_template);


}
