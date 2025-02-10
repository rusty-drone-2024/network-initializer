use super::{load_from_file, NetworkInitializer};
use crate::factory::{DroneImpl, LeafImpl};
use crate::network::Network;

impl NetworkInitializer {
    /// Initialize network with specific factories.
    /// # Returns
    /// Network according to the configuration (read from file path)
    /// # Errors
    /// - In case the topology file doesn't exist
    /// - In case the file cannot be parsed
    /// - In case the topology is invalid
    /// - In case any factory is empty
    pub fn start_network(
        config_file_path: &str,
        drone_factories: Vec<DroneImpl>,
        client_factories: Vec<LeafImpl>,
        server_factories: Vec<LeafImpl>,
    ) -> Result<Network, String> {
        let config = load_from_file(config_file_path)?;
        let ni =
            NetworkInitializer::new(&config, drone_factories, client_factories, server_factories)?;

        Ok(ni.network)
    }

    /// Initialize network with specific factories.
    /// Does not check that the topology is valid.
    /// # Returns
    /// Network according to the configuration (read from file path)
    /// # Errors
    /// - In case the topology file doesn't exist
    /// - In case the file cannot be parsed
    /// - In case any factory is empty
    pub fn unchecked_start_network(
        config_file_path: &str,
        drone_factories: Vec<DroneImpl>,
        client_factories: Vec<LeafImpl>,
        server_factories: Vec<LeafImpl>,
    ) -> Result<Network, String> {
        let config = load_from_file(config_file_path)?;
        let ni = NetworkInitializer::new_unchecked_config(
            &config,
            drone_factories,
            client_factories,
            server_factories,
        )?;

        Ok(ni.network)
    }
}
