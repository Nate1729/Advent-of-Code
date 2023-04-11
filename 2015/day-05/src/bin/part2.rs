use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_sandwich(input_string: &String) -> bool {
    let mut string_iter = input_string.chars();
    let mut back2 = string_iter.next().unwrap();
    let mut back1 = string_iter.next().unwrap();

    for current_char in string_iter {
        if back2 == current_char {
            return true;
        }
        back2 = back1;
        back1 = current_char;
    }
    false
}

struct Pair {
    values: (char, char),
    location: (usize, usize),
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
            && self.location.0 != other.location.0
            && self.location.0 != other.location.1
            && self.location.1 != other.location.0
            && self.location.1 != other.location.1
    }
}
impl Eq for Pair {}

fn has_double_double(input_string: &String) -> bool {
    let mut string_iter = input_string.chars().enumerate();
    let (mut previous_index, mut previous_element) = string_iter.next().unwrap();

    let mut pairs: Vec<Pair> = Vec::new();

    for (index, element) in string_iter {
        let pair = Pair {
            values: (previous_element, element),
            location: (previous_index, index),
        };

        if pairs.iter().any(|p| p == &pair) {
            return true;
        }

        pairs.push(pair);
        previous_index = index;
        previous_element = element;
    }

    false
}

fn is_nice(input_string: &String) -> bool {
    has_sandwich(input_string) && has_double_double(input_string)
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut nice_count = 0;
    for line in reader.lines() {
        let s = line.unwrap();
        let val = is_nice(&s);
        if val {
            nice_count += 1;
        }
        println!("{}\t{}", &s, val);
    }
    println!("Nice count: {}", nice_count);
}
