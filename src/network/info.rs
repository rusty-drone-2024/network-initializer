use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::HashSet;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct NodeInfo {
    /// Neighbor nodes of this node
    pub neighbours: HashSet<NodeId>,
    /// Channel to send packet to this node
    pub packet_in_channel: Sender<Packet>,
    /// Type of the node and its respective properties
    pub type_info: TypeInfo,
}

pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

pub struct DroneInfo {
    /// Packet Drop Rate
    pub pdr: f32,
    /// Channel to send commands to this drone
    pub command_send_channel: Sender<DroneCommand>,
}

pub struct LeafInfo {
    /// Channel to send commands to this leaf
    pub command_send_channel: Sender<LeafCommand>,
}
