mod day1;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub enum Err {
    Failed()
}

fn main() {
    let account = input_vec::<u32>("./input/day1.txt");

    if let Ok(result) = day1::first(&account) {
        println!("Day 1, part 1: {}", result);
    }

    if let Ok(result) = day1::second(&account) {
        println!("Day 1, part 2: {}", result);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn input_vec<T: std::str::FromStr>(filename: &str) -> Vec<T> {
    let mut input:Vec<T> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                if let Ok(parsed) = value.parse::<T>() {
                    input.push(parsed);
                }
            }
        }
    }

    input
}
