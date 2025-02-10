use crate::factory::{DroneImpl, LeafImpl};
use crate::network::{Network, SimulationChannels};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::config::Config;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NetworkInitializer {
    network: Network,
}

impl NetworkInitializer {
    pub fn start_simulation_from_config(
        config: &Config,
        drone_factories: Vec<DroneImpl>,
        client_factories: Vec<LeafImpl>,
        server_factories: Vec<LeafImpl>,
    ) -> Result<Network, String> {
        let ni =
            NetworkInitializer::new(config, drone_factories, client_factories, server_factories)?;
        Ok(ni.network)
    }

    fn new(
        config: &Config,
        drone_factories: Vec<DroneImpl>,
        client_factories: Vec<LeafImpl>,
        server_factories: Vec<LeafImpl>,
    ) -> Result<Self, String> {
        Self::check_config(config)?;
        Self::check_factories(&drone_factories, &client_factories, &server_factories)?;

        let mut topology = HashMap::new();
        let (drone_event_sender, drone_event_listener) = unbounded();
        let (leaf_event_sender, leaf_event_listener) = unbounded();
        let all_packet_channels = create_packet_channels(config);

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
                    &server_factories[i % server_factories.len()],
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
                    &client_factories[i % client_factories.len()],
                    &all_packet_channels,
                    leaf_event_sender.clone(),
                ),
            );
        }

        Ok(Self {
            network: Network {
                topology,
                simulation_channels: SimulationChannels {
                    drone_event_listener,
                    leaf_event_listener,
                    drone_event_sender,
                    leaf_event_sender,
                },
                drone_factories,
                client_factories,
                server_factories,
            },
        })
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
