use crate::factory::DroneImpl;

use crate::network::{DroneInfo, NodeInfo, TypeInfo};
use crate::utils::factory::{DroneEvent, NodeId, Packet};
use crossbeam_channel::{unbounded, Sender};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::thread;

#[must_use]
/// Create a single drone using a pseudo-random factory (derived from id).
pub fn create_drone(
    id: NodeId,
    pdr: f32,
    event_send: Sender<DroneEvent>,
    ngbs_packet_channels: &HashMap<NodeId, Sender<Packet>, RandomState>,
    drone_factories: &[DroneImpl],
) -> NodeInfo {
    let factory = &drone_factories[id as usize % drone_factories.len()];
    let creator = &factory.create;
    let (command_send, command_rcv) = unbounded();
    let (packet_in, packet_rcv) = unbounded();

    let mut drone = creator(
        id,
        event_send,
        command_rcv,
        packet_rcv,
        ngbs_packet_channels.clone(),
        pdr,
    );

    thread::spawn(move || drone.run());

    let type_info = TypeInfo::Drone(DroneInfo {
        pdr,
        command_send_channel: command_send,
    });
    NodeInfo {
        neighbours: ngbs_packet_channels.keys().copied().collect(),
        packet_in_channel: packet_in,
        name_impl: factory.name.clone(),
        type_info,
    }
}
