mod drone_test;
mod network_initializer;
mod utils;

use crate::network_initializer::{load_from_file, NetworkInitializer};
use common_structs::leaf::Leaf;
use wg_2024::drone::Drone;

pub use crate::utils::dummy::DummyLeaf;
pub use crate::utils::factory::*;

use ap2024_unitn_cppenjoyers_drone::CppEnjoyersDrone;
use bagel_bomber::BagelBomber;
use common_structs::network::Network;
use d_r_o_n_e_drone::MyDrone as DRONEDrone;
use dr_ones::Drone as DrOnes;
use fungi_drone::FungiDrone;
use lockheedrustin_drone::LockheedRustin;
use rustafarian_drone::RustafarianDrone;
use rustbusters_drone::RustBustersDrone;
use wg_2024_rust::drone::RustDrone;
use LeDron_James::Drone as LeDronJames;

pub fn initialize_default_network() -> Network {
    let drone_factories = drone_factories!(
        RustafarianDrone,
        DrOnes,
        FungiDrone,
        DRONEDrone,
        CppEnjoyersDrone,
        LockheedRustin,
        LeDronJames,
        BagelBomber,
        RustDrone,
        RustBustersDrone
    );

    let client_factories = leaf_factories!(DummyLeaf, DummyLeaf);
    let server_factories = leaf_factories!(DummyLeaf, DummyLeaf);

    initialize_network_with_implementation(drone_factories, client_factories, server_factories)
}

pub fn initialize_network_with_implementation(
    drone_factories: Vec<DroneFactory>,
    client_factories: Vec<LeafFactory>,
    server_factories: Vec<LeafFactory>,
) -> Network {
    let config = load_from_file("./config.toml");
    NetworkInitializer::start_simulation_from_config(
        config,
        drone_factories,
        client_factories,
        server_factories,
    )
}
