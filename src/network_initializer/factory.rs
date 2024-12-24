use crate::structs::dummy::{DummyDrone, DummyLeaf};
use crate::structs::leaf::Leaf;
use crate::structs::leaf::LeafCommand;
use crate::structs::leaf::LeafPacketSentEvent;
use crossbeam_channel::unbounded;
pub use crossbeam_channel::{Receiver, Sender};
pub use std::collections::HashMap;
use std::thread;
pub use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::drone::Drone;
pub use wg_2024::network::NodeId;
pub use wg_2024::packet::Packet;

#[allow(dead_code)]
pub trait DroneRunnable: Drone + Send {}
impl<T: Drone + Send> DroneRunnable for T {}
pub type DroneFactory = Box<
    dyn Fn(
        NodeId,
        Sender<DroneEvent>,
        Receiver<DroneCommand>,
        Receiver<Packet>,
        HashMap<NodeId, Sender<Packet>>,
        f32,
    ) -> Box<dyn DroneRunnable>,
>;

#[allow(dead_code)]
pub trait LeafRunnable: Leaf + Send {}
impl<T: Leaf + Send> LeafRunnable for T {}
pub type LeafFactory = Box<
    dyn Fn(
        NodeId,
        Sender<LeafPacketSentEvent>,
        Receiver<LeafCommand>,
        Receiver<Packet>,
        HashMap<NodeId, Sender<Packet>>,
    ) -> Box<dyn LeafRunnable>,
>;

#[macro_export]
macro_rules! drone_factories {
    ($($type_name:ty),* $(,)?) => {{
        vec![
            $(
                Box::new(
                    |id, csend, crecv, precv, psend, pdr| -> Box<dyn DroneRunnable> {
                        Box::new(<$type_name>::new(id, csend, crecv, precv, psend, pdr))
                    }
                ) as DroneFactory
            ),*
        ]
    }};
}

#[macro_export]
macro_rules! leaf_factories {
    ($($type_name:ty),* $(,)?) => {{
        vec![
            $(
                Box::new(
                    |id, csend, crecv, precv, psend| -> Box<dyn LeafRunnable> {
                        Box::new(<$type_name>::new(id, csend, crecv, precv, psend))
                    }
                ) as LeafFactory
            ),*
        ]
    }};
}

pub fn _example_drone() {
    let factories = drone_factories!(DummyDrone, DummyDrone);
    let mut join_handles = Vec::new();
    for (i, factory) in factories.iter().enumerate() {
        let mut drone = factory(
            i as NodeId,
            unbounded().0,
            unbounded().1,
            unbounded().1,
            HashMap::new(),
            0.0,
        );
        let handle = thread::spawn(move || drone.run());
        join_handles.push(handle);
    }
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}

pub fn _example_leaf() {
    let factories = leaf_factories!(DummyLeaf, DummyLeaf);
    let mut join_handles = Vec::new();
    for (i, factory) in factories.iter().enumerate() {
        let mut drone = factory(
            i as NodeId,
            unbounded().0,
            unbounded().1,
            unbounded().1,
            HashMap::new(),
        );
        let handle = thread::spawn(move || drone.run());
        join_handles.push(handle);
    }
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
