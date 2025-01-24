use crate::network::{Network, SimulationChannels};
use crate::utils::factory::DroneFactory;
use crate::utils::factory::LeafFactory;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::config::Config;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NetworkInitializer {
    network: Network,
}

impl NetworkInitializer {
    #[must_use]
    pub fn start_simulation_from_config(
        config: Config,
        drone_factories: Vec<DroneFactory>,
        client_factories: Vec<LeafFactory>,
        server_factories: Vec<LeafFactory>,
    ) -> Network {
        let ni =
            NetworkInitializer::new(config, drone_factories, client_factories, server_factories);
        ni.network
    }

    #[allow(clippy::needless_pass_by_value)]
    fn new(
        config: Config,
        drone_factories: Vec<DroneFactory>,
        client_factories: Vec<LeafFactory>,
        server_factories: Vec<LeafFactory>,
    ) -> Self {
        let mut topology = HashMap::new();
        let (drone_event_sender, drone_event_listener) = unbounded();
        let (leaf_event_sender, leaf_event_listener) = unbounded();
        let all_packet_channels = create_packet_channels(&config);

        for (i, node) in config.drone.iter().enumerate() {
            topology.insert(
                node.id,
                Self::new_drone(
                    node,
                    &drone_factories[i % drone_factories.len()],
                    &all_packet_channels,
                    drone_event_sender.clone(),
                ),
            );
        }

        for (i, node) in config.server.iter().enumerate() {
            topology.insert(
                node.id,
                Self::new_server(
                    node,
                    &server_factories[i % drone_factories.len()],
                    &all_packet_channels,
                    leaf_event_sender.clone(),
                ),
            );
        }

        for (i, node) in config.client.iter().enumerate() {
            topology.insert(
                node.id,
                Self::new_client(
                    node,
                    &client_factories[i % drone_factories.len()],
                    &all_packet_channels,
                    leaf_event_sender.clone(),
                ),
            );
        }

        Self {
            network: Network {
                topology,
                simulation_channels: SimulationChannels {
                    drone_event_listener,
                    leaf_event_listener,
                    drone_event_sender,
                    leaf_event_sender,
                },
            },
        }
    }
}

fn create_packet_channels(config: &Config) -> HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)> {
    let mut res = HashMap::new();

    for node in &config.drone {
        res.insert(node.id, unbounded());
    }
    for node in &config.server {
        res.insert(node.id, unbounded());
    }
    for node in &config.client {
        res.insert(node.id, unbounded());
    }

    res
}
