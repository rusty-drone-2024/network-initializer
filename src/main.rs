mod drone_test;
mod network_initializer;
mod structs;

use crate::network_initializer::load_from_file;
use crate::network_initializer::network::Network;

fn main() {
    let config = load_from_file("./config.toml");
    let _network = Network::start_simulation_from_config(config);
}
