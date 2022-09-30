use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32)
}

struct Position {
    depth: u32,
    h_pos: u32,
}

impl Position {
    pub fn new() -> Self {
        Position {
            depth: 0,
            h_pos: 0,
        }
    }
    
    pub fn move_command(&mut self, dir: Direction) {
        match dir {
            Direction::Forward(n) => self.h_pos += n,
            Direction::Up(n) => self.depth -= n,
            Direction::Down(n) => self.depth += n
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut pos = Position::new();
    for line in reader.lines()
    {
        let clean_line = line.unwrap();
        let data: Vec<&str> = clean_line.split(' ').collect();
        if data.len() != 2 { panic!("Help me"); }

        let distance = data[1].parse::<u32>().unwrap();
        
        match data[0] {
            "forward" => pos.move_command(Direction::Forward(distance)),
            "up" => pos.move_command(Direction::Up(distance)),
            "down" => pos.move_command(Direction::Down(distance)),
            _ => eprintln!("Error command not recognized!")
        }
        
    }

    println!("Answer is {}", pos.depth * pos.h_pos);
}
