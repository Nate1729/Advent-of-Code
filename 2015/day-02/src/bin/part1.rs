use std::fs;
use std::io::{BufRead, BufReader};

mod helpers;
use helpers::{string_to_nums};

fn calculate_wrapping_area(dimensions: &mut [u32; 3]) -> u32 {
    dimensions.sort();

    2 * (dimensions[0] * dimensions[1])
        + 2 * (dimensions[2] * dimensions[1])
        + 2 * (dimensions[0] * dimensions[2]) 
        + (dimensions[0] * dimensions[1])
}

fn main() {
    let file = fs::File::open("input.txt").expect("Error reading file");
    let reader = BufReader::new(file);

    let mut total_area: u32 = 0;
    for line in reader.lines() {
        let mut sides = string_to_nums(&line.unwrap()).unwrap();
        total_area += calculate_wrapping_area(&mut sides);
    }

    println!("The elves need {} sqft", total_area);
}
