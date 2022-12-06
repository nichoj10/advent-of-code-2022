use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn score_one(me: char, op: char) -> u32 {
    match me {
        'X' => {
            1 + match op {
                'A' => 3,
                'C' => 6,
                _ => 0,
            }
        }
        'Y' => {
            2 + match op {
                'A' => 6,
                'B' => 3,
                _ => 0,
            }
        }
        'Z' => {
            3 + match op {
                'B' => 6,
                'C' => 3,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn score_two(op: char, result: char) -> u32 {
    match result {
        // win
        'Z' => {
            6 + match op {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => 0,
            }
        }
        // tie
        'Y' => {
            3 + match op {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => 0,
            }
        }
        // lose
        'X' => match op {
            'A' => 3,
            'B' => 1,
            'C' => 2,
            _ => 0,
        },
        _ => 0,
    }
}

fn main() {
    let lines = lines_from_file("../assets/day-2-input.txt");

    let mut total = 0;

    // Part 1
    for game in &lines {
        let op = game.chars().nth(0).unwrap();
        let me = game.chars().nth(2).unwrap();
        total += score_one(me, op);
    }
    println!("Part 1: {}", total.to_string());

    // Part 2
    total = 0;
    for game in lines {
        let op = game.chars().nth(0).unwrap();
        let result = game.chars().nth(2).unwrap();
        total += score_two(op, result);
    }
    println!("Part 2: {}", total.to_string());
}
