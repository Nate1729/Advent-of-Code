use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("test_input").unwrap();
    let reader = BufReader::new(file);

    let depths: Vec<u32> = reader.lines()
        .map(|s| s.unwrap())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut previous_depth: Option<u32> = None;
    let mut n_increases = 0;   

    for depth in depths { 
        match previous_depth {
            None => (),
            Some(prev_d) => {
                if prev_d < depth { n_increases += 1; }
            }
        }

        previous_depth = Some(depth);
    }
    
    println!("There were {} increases!", n_increases);
}
