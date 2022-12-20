mod math;
mod node;
pub mod network;

use network::Network;

fn main() {
    let network = Network::new(vec![3,4,4,3]);
}
