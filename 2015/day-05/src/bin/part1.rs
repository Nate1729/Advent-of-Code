use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_three_vowels(input_string: &String) -> bool {
    input_string
        .chars()
        .filter(|s| match s {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
        > 2
}

fn pairwise_check(input_string: &String) -> bool {
    let mut string_iter = input_string.chars();
    let mut previous_char = string_iter.next().unwrap();

    let mut has_duplicate = false;
    for current_char in string_iter {
        if current_char == previous_char {
            has_duplicate = true;
        }

        match (previous_char, current_char) {
            ('a', 'b')  | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
            _ => (),
        }

        previous_char = current_char;
    }

    // If we made it this far it's up to whether or not
    // there is a duplicate
    has_duplicate
}

fn is_nice(input_string: &String) -> bool {
    pairwise_check(input_string) && has_three_vowels(input_string)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut nice_count = 0;
    for line in reader.lines() {
        if is_nice(&line.unwrap()) { nice_count += 1;}
    }

    println!("Nice count: {}", nice_count);
}
