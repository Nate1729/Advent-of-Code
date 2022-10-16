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

    let mut counter_array: [usize; BIT_SIZE] = [0; BIT_SIZE];
    let n_rows: usize = data.len();

    for d in data {
        for (index, bit) in d.chars().enumerate() {
            if bit == '1' {
                // If we increment by 2 we can skip having
                // to multiply by 2 later
                counter_array[index] += 2;
            }
        }
    } 
    let binary_array = counter_array.map(|d| d / n_rows);

    // Now we convert `counter_array` to the actual number
    let mut gamma_rate: usize = 0;
    for i in 0..BIT_SIZE {
        gamma_rate |= binary_array[i] << (BIT_SIZE-(i+1));
    }
    
    // Flip the bits but need to set left 3 to zero
    let flipper = (0b1111 << 12) ^ 0xFFFF;
    let epsilon_rate = gamma_rate ^ flipper;
    println!("  gamma rate: {:16b}", gamma_rate);
    println!("epsilon rate: {:16b}", epsilon_rate);

    let consumption_rate = gamma_rate as u32 * epsilon_rate as u32;

    println!("Consumption rate: {}", consumption_rate);
}
