use super::{load_from_file, NetworkInitializer};
use crate::factory::DroneRunnable;
use crate::factory::LeafRunnable;
use common_structs::leaf::Leaf;
use wg_2024::drone::Drone;

use crate::factory::{DroneFactory, LeafFactory};
use crate::network::Network;
use crate::{drone_factories, leaf_factories};
use ap2024_unitn_cppenjoyers_drone::CppEnjoyersDrone;
use bagel_bomber::BagelBomber;
use d_r_o_n_e_drone::MyDrone as DRONEDrone;
use dr_ones::Drone as DrOnes;
use fungi_drone::FungiDrone;
use lockheedrustin_drone::LockheedRustin;
use matteo_contribution as mc;
use rustafarian_drone::RustafarianDrone;
use rustbusters_drone::RustBustersDrone;
use rusty_drones::RustyDrone;
use rusty_drones_servers::{ChatServer, MediaServer, TextServer};
use wg_2024_rust::drone::RustDrone;
use LeDron_James::Drone as LeDronJames;

impl NetworkInitializer {
    #[must_use]
    /// Initialize network with default factories.
    /// # Returns
    /// Network according to the configuration (read from file path)
    pub fn initialize_default_network(config_file_path: &str) -> Network {
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

        let client_factories = leaf_factories!(mc::TextMediaClient);
        let server_factories = leaf_factories!(
            mc::TextServer,
            mc::MediaServer,
            TextServer,
            MediaServer,
            ChatServer
        );

        Self::initialize_network_with_implementation(
            config_file_path,
            drone_factories,
            client_factories,
            server_factories,
        )
    }

    #[must_use]
    /// Initialize network with only `RustyDrone` as drone factory.
    /// # Returns
    /// Network according to the configuration (read from file path)
    pub fn initialize_default_network_with_only_rusty_drone(config_file_path: &str) -> Network {
        let drone_factories = drone_factories!(RustyDrone);

        let client_factories = leaf_factories!(mc::TextMediaClient);
        let server_factories = leaf_factories!(mc::TextServer, mc::MediaServer);

        Self::initialize_network_with_implementation(
            config_file_path,
            drone_factories,
            client_factories,
            server_factories,
        )
    }

    #[must_use]
    /// Initialize network with specific factories.
    /// # Returns
    /// Network according to the configuration (read from file path)
    pub fn initialize_network_with_implementation(
        config_file_path: &str,
        drone_factories: Vec<DroneFactory>,
        client_factories: Vec<LeafFactory>,
        server_factories: Vec<LeafFactory>,
    ) -> Network {
        let config = load_from_file(config_file_path);
        NetworkInitializer::start_simulation_from_config(
            config,
            drone_factories,
            client_factories,
            server_factories,
        )
    }
}
