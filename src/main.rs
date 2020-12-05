mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum Err {
    Failed(),
}

fn main() {
    let matches = App::new("Advent of code 2020")
        .arg(
            Arg::with_name("day")
                .short("d")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    if let Some(a) = matches.value_of("day") {
        match a {
            "1" => {
                let account = input_vec::<u32>("./input/day1.txt");
                if let Ok(result) = day1::first(&account) {
                    println!("Day 1, part 1: {}", result);
                }

                if let Ok(result) = day1::second(&account) {
                    println!("Day 1, part 2: {}", result);
                }
            }
            "2" => {
                let passwords = input_vec::<String>("./input/day2.txt");
                if let Ok(result) = day2::first(&passwords) {
                    println!("Day 2, part 1: {}", result);
                }

                if let Ok(result) = day2::second(&passwords) {
                    println!("Day 2, part 2: {}", result);
                }
            }
            "3" => {
                let path = input_vec::<String>("./input/day3.txt");
                if let Ok(result) = day3::first(&path) {
                    println!("Day 3, part 1: {}", result);
                }

                if let Ok(result) = day3::second(&path) {
                    println!("Day 3, part 2: {}", result);
                }
            }
            "4" => {
                let passports = input_vec::<String>("./input/day4.txt");
                if let Ok(result) = day4::first(&passports) {
                    println!("Day 4, part 1: {}", result);
                }

                if let Ok(result) = day4::second(&passports) {
                    println!("Day 4, part 2: {}", result);
                }
            }
            "5" => {
                let seats = input_vec::<String>("./input/day5.txt");
                if let Ok(result) = day5::first(&seats) {
                    println!("Day 5, part 1: {}", result);
                }

                if let Ok(result) = day5::second(&seats) {
                    println!("Day 5, part 2: {}", result);
                }
            }
            &_ => {
                println!("Not there yet!");
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn input_vec<T: std::str::FromStr>(filename: &str) -> Vec<T> {
    let mut input: Vec<T> = Vec::new();

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
