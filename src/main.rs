mod tester1;
mod tester3;
mod tester2;

use rusty_drones::RustyDrone;
use getdroned::GetDroned;
use wg_2024::drone::Drone;
use rolling_drone::RollingDrone;

fn main() {
    get(0);
    get(1);

}

fn get(a: i32){
    if a > 0 {
        GetDroned::new(todo!(),todo!(), todo!(),todo!(), todo!(), todo!());
    }else if a == 0 {
        RustyDrone::new(todo!(),todo!(),todo!(),todo!(),todo!(),todo!());
    } else {
        RollingDrone::new(todo!(),todo!(),todo!(),todo!(),todo!(),todo!());
    }
}