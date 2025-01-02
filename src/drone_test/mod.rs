#![cfg(test)]
use std::time::Duration;

mod bagel_bomber;
mod cpp_enjoyers;
mod d_r_o_n_e;
mod dr_ones;
mod fungi;
mod le_drones;
mod lockheed;
mod rust;
mod rustafarian;
mod rustbusters;

const TIMEOUT: Duration = Duration::from_millis(50);
const FLOOD_TIMEOUT: Duration = Duration::from_millis(300);
const HARD_TEST_TIMEOUT: Duration = Duration::from_millis(2000);
