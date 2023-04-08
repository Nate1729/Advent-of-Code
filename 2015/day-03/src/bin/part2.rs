use std::fs;

use day_03::helpers::{Location, add_location};

enum Turn {
    Santa,
    RoboSanta
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut santa = Location::new();
    let mut robo_santa = Location::new();
    
    let mut turn = Turn::Santa;

    let mut locations: Vec<Location> = Vec::new();
    add_location(&mut locations, santa.clone());

    for c in input.chars() {
        match turn  {
            Turn::Santa => {
                santa.move_by_char(c);
                add_location(&mut locations, santa.clone());
                turn = Turn::RoboSanta;
            }
            Turn::RoboSanta => {
                robo_santa.move_by_char(c);
                add_location(&mut locations, robo_santa.clone());
                turn = Turn::Santa;
            }
        }
    }

    println!("Total visited houses: {}", locations.len());
}
