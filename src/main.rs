extern crate rand;
use std::thread;
use std::time;

fn main() {
    let mut world = [[0u8; 75]; 75];
    let mut generations = 0;

    for i in 0..74 {
        for j in 0..74{
            if rand::random(){
                world[i][j] = 1;
            }else {
                world[i][j] = 0;
            }
        }
    }
}

/// A function that is used to calculate the number of organisms in the world
fn census(_world: [[u8; 75]; 75]) -> u16{
    let mut count = 0;

    for i in 0..74{
        for j in 0..74{
            if _world[i][j] == 1{
                count += 1;
            }
        }
    }
    count
}
