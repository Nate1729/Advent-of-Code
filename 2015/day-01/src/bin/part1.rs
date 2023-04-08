use std::fs;

fn process_file(input: &String) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }

    floor
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    let final_floor = process_file(&input);

    println!("Final floor: {}", final_floor);
}
