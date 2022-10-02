use std::fs::File;
use std::io::{BufReader, BufRead};

// Power Consumption = gamma_rate * epsilon_rate
// 
// gamma_rate the most common number in that slot

const BIT_SIZE: usize = 12;

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines()
        .map(|s| s.unwrap())
        .collect();

    let mut counter_array: [i32; BIT_SIZE] = [0; BIT_SIZE];

    for d in data {
        for (index, bit) in d.chars().enumerate() {
            if bit == '0' { counter_array[index] -= 1; }
            else if bit == '1' {counter_array[index] += 1;}
            else {panic!("Unknown input {}", bit);}
        }
    }
    
    // Now we convert `counter_array` to the actual number
    let mut gamma_rate: u16 = 0;
    for i in 0..BIT_SIZE {
        if counter_array[i] > 0 {
            gamma_rate |= 1 << (BIT_SIZE-(i+1));
        } else if counter_array[i] == 0 {
            eprintln!("Error! same bits in position: {}", i);
        }
    }
    
    // Flip the bits but need to set left 3 to zero
    let flipper = (0b1111 << 12) ^ 0xFFFF;
    let epsilon_rate = gamma_rate ^ flipper;
    println!("  gamma rate: {:16b}", gamma_rate);
    println!("epsilon rate: {:16b}", epsilon_rate);

    let consumption_rate = gamma_rate as u32 * epsilon_rate as u32;

    println!("Consumption rate: {}", consumption_rate);
}
