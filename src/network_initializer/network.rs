use crate::network_initializer::info::NodeInfo;
use crossbeam_channel::{unbounded, Receiver, Sender};
use rusty_drones::RustyDrone;
use std::collections::HashMap;
use wg_2024::config::Config;
use wg_2024::controller::DroneEvent;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

#[allow(dead_code)]
pub struct Network {
    topology: HashMap<NodeId, NodeInfo>,
    simulation_channels: SimulationChannels,
}

#[allow(dead_code)]
pub struct SimulationChannels {
    sc_event_listener: Receiver<DroneEvent>,
    sc_event_sender: Sender<DroneEvent>,
}

impl Network {
    pub fn start_simulation_from_config(config: Config) -> Self {
        Network::new(config)
    }

    fn new(config: Config) -> Self {
        let mut topology = HashMap::new();
        let (sc_event_sender, sc_event_listener) = unbounded();
        let all_packet_channels = create_packet_channels(&config);
        //TODO various impl for all

        for node in config.drone {
            topology.insert(
                node.id,
                NodeInfo::new_drone::<RustyDrone>(
                    node,
                    &all_packet_channels,
                    sc_event_sender.clone(),
                ),
            );
        }

        for node in config.server {
            topology.insert(node.id, NodeInfo::new_server(node));
        }

        for node in config.client {
            topology.insert(node.id, NodeInfo::new_client(node));
        }

        Self {
            topology,
            simulation_channels: SimulationChannels {
                sc_event_listener,
                sc_event_sender,
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
