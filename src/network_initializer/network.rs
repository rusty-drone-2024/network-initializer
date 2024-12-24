use crate::network_initializer::factory::DroneFactory;
use crate::network_initializer::factory::DroneRunnable;
use crate::network_initializer::factory::LeafFactory;
use crate::network_initializer::factory::LeafRunnable;
use crate::network_initializer::info::NodeInfo;
use crate::structs::dummy::{DummyDrone, DummyLeaf};
use crate::structs::leaf::Leaf;
use crate::structs::leaf::LeafPacketSentEvent;
use crate::{drone_factories, leaf_factories};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use wg_2024::config::Config;
use wg_2024::controller::DroneEvent;
use wg_2024::drone::Drone;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

#[allow(dead_code)]
pub struct Network {
    pub topology: HashMap<NodeId, NodeInfo>,
    pub simulation_channels: SimulationChannels,
}

#[allow(dead_code)]
pub struct SimulationChannels {
    pub drone_event_listener: Receiver<DroneEvent>,
    pub drone_event_sender: Sender<DroneEvent>,
    pub leaf_event_listener: Receiver<LeafPacketSentEvent>,
    pub leaf_event_sender: Sender<LeafPacketSentEvent>,
}

impl Network {
    pub fn start_simulation_from_config(config: Config) -> Self {
        Network::new(config)
    }

    fn new(config: Config) -> Self {
        let mut topology = HashMap::new();
        let (drone_event_sender, drone_event_listener) = unbounded();
        let (leaf_event_sender, leaf_event_listener) = unbounded();
        let all_packet_channels = create_packet_channels(&config);

        let drone_factories = drone_factories!(DummyDrone, DummyDrone);
        let client_factories = leaf_factories!(DummyLeaf, DummyLeaf);
        let server_factories = leaf_factories!(DummyLeaf, DummyLeaf);

        for (i, node) in config.drone.iter().enumerate() {
            topology.insert(
                node.id,
                NodeInfo::new_drone(
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
                NodeInfo::new_server(
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
                NodeInfo::new_client(
                    node,
                    &client_factories[i % drone_factories.len()],
                    &all_packet_channels,
                    leaf_event_sender.clone(),
                ),
            );
        }

        Self {
            topology,
            simulation_channels: SimulationChannels {
                drone_event_listener,
                drone_event_sender,
                leaf_event_listener,
                leaf_event_sender,
            },
        }
    }
}

fn create_packet_channels(config: &Config) -> HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)> {
    let mut res = HashMap::new();

    for node in config.drone.iter() {
        res.insert(node.id, unbounded());
    }
    for node in config.server.iter() {
        res.insert(node.id, unbounded());
    }
    for node in config.client.iter() {
        res.insert(node.id, unbounded());
    }

    res
}
