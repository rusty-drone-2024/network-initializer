use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::{HashMap, HashSet};
use std::thread;
use wg_2024::config;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::drone::Drone;
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
    //event_listener_channel: Receiver<DroneEvent>, TODO make 1 global
}

pub struct LeafInfo {
    //event_listener_channel: Receiver<DroneEvent>, TODO create type
    //command_send_channel: Sender<DroneCommand>, TODO create type
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

    pub fn new_drone<T: Drone>(
        data: config::Drone,
        all_packet_channels: &HashMap<NodeId, (Sender<Packet>, Receiver<Packet>)>,
        event_send: Sender<DroneEvent>,
    ) -> Self {
        let (command_send, command_rcv) = unbounded();
        let packet_send = filter_hashmap_sender(all_packet_channels, &data.connected_node_ids);
        let (packet_in, packet_rcv) = all_packet_channels[&data.id].clone();

        thread::spawn(move || {
            T::new(
                data.id,
                event_send,
                command_rcv,
                packet_rcv,
                packet_send,
                data.pdr,
            )
            .run();
        });

        let type_info = TypeInfo::Drone(DroneInfo {
            pdr: data.pdr,
            command_send_channel: command_send,
        });
        NodeInfo::new(data.connected_node_ids, type_info, packet_in)
    }

    pub fn new_client(_data: config::Client) -> Self {
        todo!()
    }

    pub fn new_server(_data: config::Server) -> Self {
        todo!()
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
