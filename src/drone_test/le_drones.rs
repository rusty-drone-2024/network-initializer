#![cfg(test)]
use crate::drone_test::{FLOOD_TIMEOUT, HARD_TEST_TIMEOUT, TIMEOUT};
use rusty_tester::*;

type Tested = LeDron_James::Drone;

#[test]
fn drone_destination_is_drone() {
    test_drone_destination_is_drone::<Tested>(TIMEOUT);
}

#[test]
fn drone_error_in_routing() {
    test_drone_error_in_routing::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_1_hop() {
    test_drone_packet_1_hop::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_3_hop() {
    test_drone_packet_3_hop::<Tested>(TIMEOUT);
}

#[test]
fn drone_packet_3_hop_crash() {
    test_drone_packet_3_hop_crash::<Tested>(TIMEOUT);
}

#[test]
#[ignore]
fn drone_packet_255_hop() {
    test_drone_packet_255_hop::<Tested>(HARD_TEST_TIMEOUT);
}

#[test]
fn easiest_flood() {
    test_easiest_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn loop_flood() {
    test_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn hard_loop_flood() {
    test_hard_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn matrix_loop_flood() {
    test_matrix_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn star_loop_flood() {
    test_star_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn butterfly_loop_flood() {
    test_butterfly_loop_flood::<Tested>(FLOOD_TIMEOUT);
}

#[test]
fn tree_loop_flood() {
    test_tree_loop_flood::<Tested>(FLOOD_TIMEOUT);
}