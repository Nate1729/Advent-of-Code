use std::fs::File;
use std::io::{BufRead, BufReader};

mod helpers;
use helpers::string_to_nums;

fn get_ribbon_length(sides: &mut [u32; 3]) -> u32 {
    sides.sort();

    let ribbon = 2*(sides[0] + sides[1]);
    let bow = sides[0] * sides[1] * sides[2];

    ribbon + bow
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_length: u32 = 0;
    for line in reader.lines() {
        let mut sides = string_to_nums(&line.unwrap()).unwrap();

        total_length += get_ribbon_length(&mut sides);
    }

    println!("Total ribbon length: {}", total_length);
}
