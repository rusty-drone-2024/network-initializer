mod info;

use crate::factory::{DroneEvent, DroneImpl, LeafImpl, NodeId};
use common_structs::leaf::LeafEvent;
use crossbeam_channel::{Receiver, Sender};
pub use info::*;
use std::collections::HashMap;

pub struct Network {
    /// All nodes in the network
    pub topology: HashMap<NodeId, NodeInfo>,
    /// Communication channels
    pub simulation_channels: SimulationChannels,
    pub drone_factories: Vec<DroneImpl>,
    pub client_factories: Vec<LeafImpl>,
    pub server_factories: Vec<LeafImpl>,
}

pub struct SimulationChannels {
    // Used by drones for communication with the Simulation Controller
    pub drone_event_listener: Receiver<DroneEvent>,
    // Used by leafs (clients and servers) for communication with the Simulation Controller
    pub leaf_event_listener: Receiver<LeafEvent>,
    // Sent to the drones of the network by the Network Initializer
    pub drone_event_sender: Sender<DroneEvent>,
    // Sent to the leafs (clients and servers) of the network by the Network Initializer
    pub leaf_event_sender: Sender<LeafEvent>,
}
