mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc::input_vec;
use clap::{App, Arg};

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
            "6" => {
                let answers = input_vec::<String>("./input/day6.txt");
                if let Ok(result) = day6::first(&answers) {
                    println!("Day 6, part 1: {}", result);
                }

                if let Ok(result) = day6::second(&answers) {
                    println!("Day 6, part 2: {}", result);
                }
            }
            "7" => {
                let bags = input_vec::<String>("./input/day7.txt");
                if let Ok(result) = day7::first(&bags) {
                    println!("Day 7, part 1: {}", result);
                }

                if let Ok(result) = day7::second(&bags) {
                    println!("Day 7, part 2: {}", result);
                }
            }
            "8" => {
                let instructions = input_vec::<String>("./input/day8.txt");
                if let Ok(result) = day8::first(&instructions) {
                    println!("Day 8, part 1: {}", result);
                }

                if let Ok(result) = day8::second(&instructions) {
                    println!("Day 8, part 2: {}", result);
                }
            }
            "9" => {
                let numbers = input_vec::<u128>("./input/day9.txt");
                if let Ok(result) = day9::first(&numbers, 25) {
                    println!("Day 9, part 1: {}", result);

                    if let Ok(result) = day9::second(&numbers, result) {
                        println!("Day 9, part 2: {}", result);
                    }
                }
            }
            "10" => {
                let mut adapters = input_vec::<u64>("./input/day10.txt");
                if let Ok(result) = day10::first(adapters.as_mut_slice()) {
                    println!("Day 10, part 1: {}", result);
                }

                if let Ok(result) = day10::second(adapters.as_mut_slice()) {
                    println!("Day 10, part 2: {}", result);
                }
            }
            &_ => {
                println!("Not there yet!");
            }
        }
    }
}
