use common_structs::leaf::{Leaf, LeafCommand, LeafEvent};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use wg_2024::network::NodeId;
use wg_2024::packet::Packet;

pub struct DummyLeaf {}

impl Leaf for DummyLeaf {
    fn new(
        _id: NodeId,
        _controller_send: Sender<LeafEvent>,
        _controller_recv: Receiver<LeafCommand>,
        _packet_recv: Receiver<Packet>,
        _packet_send: HashMap<NodeId, Sender<Packet>>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn run(&mut self) {}
}
