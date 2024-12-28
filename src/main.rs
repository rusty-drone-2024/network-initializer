mod drone_test;
mod network_initializer;
mod utils;

use crate::network_initializer::{load_from_file, NetworkInitializer};
use crate::utils::dummy::{DummyDrone, DummyLeaf};
use crate::utils::factory::*;
use common_structs::leaf::Leaf;
use wg_2024::drone::Drone;

fn main() {
    let drone_factories = drone_factories!(DummyDrone, DummyDrone);
    let client_factories = leaf_factories!(DummyLeaf, DummyLeaf);
    let server_factories = leaf_factories!(DummyLeaf, DummyLeaf);

    let config = load_from_file("./config.toml");
    let _network = NetworkInitializer::start_simulation_from_config(
        config,
        drone_factories,
        client_factories,
        server_factories,
    );
    // RustySimulationController::start(_network);
}
