mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use clap::{App, Arg};
use aoc::input_vec;

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
            &_ => {
                println!("Not there yet!");
            }
        }
    }
}
