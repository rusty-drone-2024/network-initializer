use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub trait Leaf {
    fn new(
        id: NodeId,
        controller_send: Sender<LeafPacketSentEvent>,
        controller_recv: Receiver<LeafCommand>,
        packet_recv: Receiver<Packet>,
        packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized;

    fn run(&mut self);
}

pub type LeafPacketSentEvent = Packet;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LeafCommand {
    RemoveSender(NodeId),
    AddSender(NodeId, Sender<Packet>),
    Crash,
}
