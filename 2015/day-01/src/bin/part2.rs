use std::fs;

fn process_file(input: &String) -> Option<usize> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor == -1 {
            return Some(i);
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    match process_file(&input) {
        None => eprintln!("Never reached floor -1!"),
        Some(index) => println!("First reached floor -1 at index {}", index + 1),
    }
}
