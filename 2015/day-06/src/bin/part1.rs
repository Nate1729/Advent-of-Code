use std::fs::File;
use std::io::{BufRead, BufReader};

enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_string(s: &str) -> Self {
        let split: Vec<&str> = s.split(",").collect();
        Point {
            x: split[0].parse::<usize>().unwrap(),
            y: split[1].parse::<usize>().unwrap(),
        }
    }
}

const DIM: usize = 1000;

fn process_string(s: String) -> (LightAction, Point, Point) {
    let mut s_split = s.split(" ");

    let light_action = if s_split.next().unwrap() == "toggle" {
        LightAction::Toggle
    } else {
        if s_split.next().unwrap() == "on" {
            LightAction::TurnOn
        } else {
            LightAction::TurnOff
        }
    };

    let p1 = Point::from_string(s_split.next().unwrap());
    s_split.next().unwrap();
    let p2 = Point::from_string(s_split.next().unwrap());

    (light_action, p1, p2)
}


fn update_grid(action: LightAction, p1: Point, p2: Point, grid: &mut [[bool; DIM]; DIM]) -> () {
    for x in p1.x..=p2.x {
        for y in p1.y..=p2.y {
            grid[x][y] = match action {
                LightAction::TurnOn => true,
                LightAction::TurnOff => false,
                LightAction::Toggle => !grid[x][y]
            }
        }
    }
}

fn count_lights_on(grid: &[[bool; DIM]; DIM]) -> usize {
    let mut total = 0;
    for x in 0..DIM {
        for y in 0..DIM {
            if grid[x][y] { total += 1; }
        }
    }

    total
}
fn main() {
    let mut grid: [[bool; DIM]; DIM] = [[false; DIM]; DIM];

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines() {
        let (light_action, p1, p2) = process_string(line.unwrap());
        update_grid(light_action, p1, p2, &mut grid)
    }

    println!("There are {} lights on!", count_lights_on(&grid));
}
