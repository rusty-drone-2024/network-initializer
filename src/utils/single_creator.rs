
use crate::factory::DroneRunnable;
use wg_2024::drone::Drone;

use crate::network::{DroneInfo, NodeInfo, TypeInfo};
use crate::utils::factory::{DroneEvent, DroneFactory, NodeId, Packet};
use crossbeam_channel::{unbounded, Sender};
use std::collections::HashMap;
use std::thread;

use crate::drone_factories;
use ap2024_unitn_cppenjoyers_drone::CppEnjoyersDrone;
use bagel_bomber::BagelBomber;
use d_r_o_n_e_drone::MyDrone as DRONEDrone;
use dr_ones::Drone as DrOnes;
use fungi_drone::FungiDrone;
use lockheedrustin_drone::LockheedRustin;
use rustafarian_drone::RustafarianDrone;
use rustbusters_drone::RustBustersDrone;
use wg_2024_rust::drone::RustDrone;
use LeDron_James::Drone as LeDronJames;

pub fn create_drone(id: NodeId, pdr: f32, event_send: Sender<DroneEvent>, ngbs_packet_channels: &HashMap<NodeId, Sender<Packet>>) -> NodeInfo {
    let drone_factories = drone_factories!(
        RustafarianDrone,
        DrOnes,
        FungiDrone,
        DRONEDrone,
        CppEnjoyersDrone,
        LockheedRustin,
        LeDronJames,
        BagelBomber,
        RustDrone,
        RustBustersDrone
    );
    run_drone(&drone_factories[id as usize % drone_factories.len()], id, pdr, event_send, ngbs_packet_channels)
}

fn run_drone (factory: &DroneFactory, id: NodeId, pdr: f32, event_send: Sender<DroneEvent>, ngbs_packet_channels: &HashMap<NodeId, Sender<Packet>>) -> NodeInfo {
    let (command_send, command_rcv) = unbounded();
    let (packet_in, packet_rcv) = unbounded();

    let mut drone = factory(
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
        neighbours: ngbs_packet_channels.keys().cloned().collect(),
        packet_in_channel: packet_in,
        type_info,
    }
}