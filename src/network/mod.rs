mod info;

use crate::factory::{DroneEvent, NodeId};
use common_structs::leaf::LeafEvent;
use crossbeam_channel::{Receiver, Sender};
pub use info::*;
use std::collections::HashMap;

pub struct Network {
    /// All nodes in the network
    pub topology: HashMap<NodeId, NodeInfo>,
    /// Communication channels
    pub simulation_channels: SimulationChannels,
}

pub struct SimulationChannels {
    // Used for communication with the Simulation Controller
    pub drone_event_listener: Receiver<DroneEvent>,
    pub leaf_event_listener: Receiver<LeafEvent>,
    // Sent to the nodes of the network by the Network Initializer
    pub drone_event_sender: Sender<DroneEvent>,
    pub leaf_event_sender: Sender<LeafEvent>,
}
