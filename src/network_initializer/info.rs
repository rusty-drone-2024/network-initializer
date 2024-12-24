use crate::network_initializer::factory::{DroneFactory, LeafFactory};
use crate::structs::leaf::{LeafCommand, LeafPacketSentEvent};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::thread;
use wg_2024::config;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

#[allow(dead_code)]
pub struct NodeInfo {
    neighbours: HashSet<NodeId>,
    packet_in_channel: Sender<Packet>,
    type_info: TypeInfo,
}

#[allow(dead_code)]
pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

#[allow(dead_code)]
pub struct DroneInfo {
    pdr: f32,
    command_send_channel: Sender<DroneCommand>,
}

#[allow(dead_code)]
pub struct LeafInfo {
    command_send_channel: Sender<LeafCommand>,
}

impl NodeInfo {
    fn new(
        neighbours: Vec<NodeId>,
        type_info: TypeInfo,
        packet_in_channel: Sender<Packet>,
    ) -> Self {
        Self {
            neighbours: neighbours.into_iter().collect(),
            packet_in_channel,
            type_info,
        }
    }

    pub fn new_drone(
        data: &config::Drone,
        factory: &DroneFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<DroneEvent>,
    ) -> Self {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_node_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut drone = factory(
            data.id,
            event_send,
            command_rcv,
            packet_rcv,
            packet_send,
            data.pdr,
        );

        thread::spawn(move || drone.run());

        let type_info = TypeInfo::Drone(DroneInfo {
            pdr: data.pdr,
            command_send_channel: command_send,
        });
        NodeInfo::new(data.connected_node_ids.clone(), type_info, packet_in)
    }

    pub fn new_client(
        data: &config::Client,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafPacketSentEvent>,
    ) -> Self {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_drone_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut leaf = factory(data.id, event_send, command_rcv, packet_rcv, packet_send);

        thread::spawn(move || {
            leaf.run();
        });

        let type_info = TypeInfo::Client(LeafInfo {
            command_send_channel: command_send,
        });
        NodeInfo::new(data.connected_drone_ids.clone(), type_info, packet_in)
    }

    pub fn new_server(
        data: &config::Server,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafPacketSentEvent>,
    ) -> Self {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_drone_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        let mut leaf = factory(data.id, event_send, command_rcv, packet_rcv, packet_send);

        thread::spawn(move || {
            leaf.run();
        });

        let type_info = TypeInfo::Server(LeafInfo {
            command_send_channel: command_send,
        });
        NodeInfo::new(data.connected_drone_ids.clone(), type_info, packet_in)
    }
}

fn filter_hashmap_sender(
    all: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
    filter: &Vec<NodeId>,
) -> HashMap<NodeId, Sender<Packet>> {
    let mut res = HashMap::new();

    for k in filter {
        res.insert(*k, all[k].0.clone());
    }

    res
}
