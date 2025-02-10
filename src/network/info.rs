use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::HashSet;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

#[allow(clippy::module_name_repetitions)]
/// Node connections, channel, and type
pub struct NodeInfo {
    /// Neighbor nodes of this node
    pub neighbours: HashSet<NodeId>,
    /// Channel to send packet to this node
    pub packet_in_channel: Sender<Packet>,
    /// Type of the node and its respective properties
    pub type_info: TypeInfo,
}

#[allow(clippy::module_name_repetitions)]
/// Different types of nodes in the network
pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

#[allow(clippy::module_name_repetitions)]
/// Information about a drone in the network.
pub struct DroneInfo {
    /// Packet Drop Rate
    pub pdr: f32,
    /// Channel to send commands to this drone
    pub command_send_channel: Sender<DroneCommand>,
}

#[allow(clippy::module_name_repetitions)]
/// Information about a leaf (client/server) in the network
pub struct LeafInfo {
    /// Channel to send commands to this leaf
    pub command_send_channel: Sender<LeafCommand>,
}
