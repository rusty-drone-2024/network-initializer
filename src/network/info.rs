use common_structs::leaf::LeafCommand;
use crossbeam_channel::Sender;
use std::collections::HashSet;
use wg_2024::controller::DroneCommand;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

/// Node connections, channel, and type
pub struct NodeInfo {
    /// Neighbor nodes of this node
    pub neighbours: HashSet<NodeId>,
    /// Channel to send packet to this node
    pub packet_in_channel: Sender<Packet>,
    pub name_impl: String,
    /// Type of the node and its respective properties
    pub type_info: TypeInfo,
}

/// Different types of nodes in the network
pub enum TypeInfo {
    Client(LeafInfo),
    Server(LeafInfo),
    Drone(DroneInfo),
}

/// Information about a drone in the network.
pub struct DroneInfo {
    /// Packet Drop Rate
    pub pdr: f32,
    /// Channel to send commands to this drone
    pub command_send_channel: Sender<DroneCommand>,
}

/// Information about a leaf (client/server) in the network
pub struct LeafInfo {
    /// Channel to send commands to this leaf
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
