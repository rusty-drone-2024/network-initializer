use super::{load_from_file, NetworkInitializer};
use crate::factory::{DroneImpl, LeafImpl};
use crate::network::Network;

impl NetworkInitializer {
    pub fn initialize_network_with_implementation(
        config_file_path: &str,
        drone_factories: Vec<DroneImpl>,
        client_factories: Vec<LeafImpl>,
        server_factories: Vec<LeafImpl>,
    ) -> Result<Network, String> {
        let config = load_from_file(config_file_path);
        NetworkInitializer::start_simulation_from_config(
            &config,
            drone_factories,
            client_factories,
            server_factories,
        )
    }
}
