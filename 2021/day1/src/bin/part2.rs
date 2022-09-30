use std::fs::File;
use std::io::{BufReader, BufRead};


pub struct WindowCache {
    content: [u32; 4],
    pub increases: u32,
}

impl WindowCache {
    pub fn new() -> Self {
        WindowCache {
            content: [0; 4],
            increases: 0,
        }
    }
    
    pub fn add_num(&mut self, new_num: u32)  {
        // Not Full yet condition
        for i in 0..4 {
            if self.content[i] == 0 {
                self.content[i] = new_num;
                return;
            }
        }

        // Full condition
        self.check_increase();

        self.content[0] = self.content[1];
        self.content[1] = self.content[2];
        self.content[2] = self.content[3];
        self.content[3] = new_num;
    }

    pub fn check_increase(&mut self) {
        if self.content[3] > self.content[0] {
            self.increases += 1;
        }
    }
}

fn main() {
    let file = File::open("test_input").unwrap();
    let reader = BufReader::new(file); 

    let mut cache = WindowCache::new();

    for line in reader.lines() {
        let num = line.unwrap().parse::<u32>().unwrap();
        cache.add_num(num);
    }

    // Need to check last window manually since we aren't adding a number
    cache.check_increase();

    println!("There were {} windowed-increases", cache.increases);
}
