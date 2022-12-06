use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Winners {
    first: u32,
    second: u32,
    third: u32,
}

impl Winners {
    pub fn new() -> Winners {
        Self {
            first: 0,
            second: 0,
            third: 0,
        }
    }

    pub fn insert(&mut self, candidate: u32) {
        if candidate > self.first {
            self.third = self.second;
            self.second = self.first;
            self.first = candidate;
        } else if candidate > self.second {
            self.third = self.second;
            self.second = candidate;
        } else if candidate > self.third {
            self.third = candidate;
        }
    }

    pub fn sum(&self) -> u32 {
        self.first + self.second + self.third
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file("../assets/day-1-input.txt");

    let mut curr = 0;
    let mut winners = Winners::new();
    for line in lines {
        if line == "" {
            winners.insert(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>().unwrap();
        }
    }

    println!("1st place: {}", winners.first);
    println!("2st place: {}", winners.second);
    println!("3rd place: {}", winners.third);
    println!("Total: {}", winners.sum());
}
