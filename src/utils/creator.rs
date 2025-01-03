use crate::utils::factory::{DroneEvent, DroneFactory, LeafFactory, NodeId, Packet};
use common_structs::leaf::LeafPacketSentEvent;
use common_structs::network::{DroneInfo, LeafInfo, NodeInfo, TypeInfo};
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread;
use wg_2024::config;

pub struct Creator {}

impl Creator {
    fn new_info(
        neighbours: Vec<NodeId>,
        type_info: TypeInfo,
        packet_in_channel: Sender<Packet>,
    ) -> NodeInfo {
        NodeInfo {
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
    ) -> NodeInfo {
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
        Creator::new_info(data.connected_node_ids.clone(), type_info, packet_in)
    }

    pub fn new_client(
        data: &config::Client,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafPacketSentEvent>,
    ) -> NodeInfo {
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
        Creator::new_info(data.connected_drone_ids.clone(), type_info, packet_in)
    }

    pub fn new_server(
        data: &config::Server,
        factory: &LeafFactory,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<LeafPacketSentEvent>,
    ) -> NodeInfo {
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
        Creator::new_info(data.connected_drone_ids.clone(), type_info, packet_in)
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
