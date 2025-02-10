use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::HashSet;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NodeInfo {
    pub neighbours: HashSet<NodeId>,
    pub packet_in_channel: Sender<Packet>,
    pub name_impl: String,
    pub type_info: TypeInfo,
}

pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

pub struct DroneInfo {
    pub pdr: f32,
    pub command_send_channel: Sender<DroneCommand>,
}

pub struct LeafInfo {
    pub command_send_channel: Sender<LeafCommand>,
}

impl NodeInfo {
    pub(crate) fn new(
        neighbours: Vec<NodeId>,
        type_info: TypeInfo,
        name_impl: String,
        packet_in_channel: Sender<Packet>,
    ) -> NodeInfo {
        NodeInfo {
            neighbours: neighbours.into_iter().collect(),
            packet_in_channel,
            name_impl,
            type_info,
        }
    }
}
