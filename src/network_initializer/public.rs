use super::{load_from_file, NetworkInitializer};
use crate::factory::{DroneFactory, LeafFactory};
use crate::network::Network;

impl NetworkInitializer {
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
