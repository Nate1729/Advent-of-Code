use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Instruction {
    Signal(u16),
    Operation(Operator)
}

#[derive(Debug)]
enum Operator {
    And(String, String),
    Not(String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Pass(String),
}

fn to_operation<'a>(input: &'a str) -> Instruction {
    let parts: Vec<&str> = input.trim().split(" ").collect();
    
    if parts.len() == 1 {
        Instruction::Operation(Operator::Pass(parts[0].to_string()))
    }
    else if parts.len() == 2 {
        // Has to be the not operator
        Instruction::Operation(Operator::Not(parts[1].to_string()))
    } else if parts[1] == "AND" {
        Instruction::Operation(Operator::And(parts[0].to_string(), parts[2].to_string()))
    } else if parts[1] == "OR" {
        Instruction::Operation(Operator::Or(parts[0].to_string(), parts[2].to_string()))
    } else if parts[1] == "LSHIFT" {
        Instruction::Operation(Operator::LShift(parts[0].to_string(), parts[2].parse::<usize>().unwrap()))
    } else if parts[1] == "RSHIFT" {
        Instruction::Operation(Operator::RShift(parts[0].to_string(), parts[2].parse::<usize>().unwrap()))
    } else {
        panic!("I don't know what to do with command {}", parts[1]);
    }
}

fn get_values(command: String) -> (Instruction, String) {
    let splitter: Vec<&str> = command.split("->").collect();

    let wire = splitter[1].trim();
    
    let instruction = match splitter[0].trim().parse::<u16>() {
        Ok(number) => Instruction::Signal(number),
        Err(_) => {
            to_operation(splitter[0])
        }
    };

    (instruction, wire.to_string())
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    
    let mut definitions: HashMap<String, Instruction> = HashMap::new();

    for line in reader.lines() {
        let (instruction, wire) = get_values(line.unwrap());

        match definitions.insert(wire.to_string(), instruction) {
            None => (),
            Some(_) => eprintln!("Hey! \"{}\" was already assigned", wire),
        }
    }
}
