use std::fs;

use day_03::helpers::{Location, add_location};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut visited_locations: Vec<Location> = Vec::new();

    let mut current_position = Location::new();
    add_location(&mut visited_locations, current_position.clone());
    for c in input.chars() {
        current_position.move_by_char(c);
        add_location(&mut visited_locations, current_position.clone());
    }

    println!("Total visited locations {}", visited_locations.len());

}
