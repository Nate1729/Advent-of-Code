use std::fs::File;
use std::io::{BufReader, BufRead};

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32)
}

struct Position {
    depth: u32,
    h_pos: u32,
    aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Position {depth:0, h_pos:0, aim:0}
    }
    
    pub fn move_command(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(n) => {
                self.h_pos += n;
                self.depth += (n as i32 * self.aim) as u32;
            }
            Direction::Up(n) => {
                self.aim -= n as i32;
            }
            Direction::Down(n) => {
                self.aim += n as i32;
            }
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pos = Position::new();
    for line in reader.lines() {
        let clean_line = line.unwrap();

        let command: Vec<&str> = clean_line.split(' ').collect();
        if command.len() != 2 {panic!("I got a weird command");}

        let dist = command[1].parse::<u32>().unwrap();

        match command[0] {
            "forward" => pos.move_command(Direction::Forward(dist)),
            "up" => pos.move_command(Direction::Up(dist)),
            "down" => pos.move_command(Direction::Down(dist)),
            e => eprintln!("Error command: {} not recognized!", e)
        }
    }

    println!("The answer is {}", pos.depth * pos.h_pos);
}
