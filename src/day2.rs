use crate::Err;
use regex::Regex;
use std::ops::Index;
use std::str::FromStr;

pub fn first(input: &[String]) -> Result<u32, Err> {
    let mut valid_password_count: u32 = 0;
    for o in input {
        if let Ok(parsed_input) = Input::from_str(o) {
            let password: Vec<char> = parsed_input.password.chars().collect();

            let mut count: u32 = 0;
            for p in password {
                if p == parsed_input.letter {
                    count += 1;
                }
            }

            if count >= parsed_input.min && count <= parsed_input.max {
                valid_password_count += 1;
            }
        }
    }

    Ok(valid_password_count)
}

pub fn second(input: &[String]) -> Result<u32, Err> {
    let mut valid_password_count: u32 = 0;

    for o in input {
        if let Ok(parsed_input) = Input::from_str(o) {
            let password: Vec<char> = parsed_input.password.chars().collect();

            if (password[(parsed_input.min - 1) as usize] == parsed_input.letter
                && password[(parsed_input.max - 1) as usize] != parsed_input.letter)
                || (password[(parsed_input.min - 1) as usize] != parsed_input.letter
                    && password[(parsed_input.max - 1) as usize] == parsed_input.letter)
            {
                valid_password_count += 1;
            }
        }
    }

    Ok(valid_password_count)
}

#[derive(Debug)]
struct Input {
    pub min: u32,
    pub max: u32,
    pub letter: char,
    pub password: String,
}

impl FromStr for Input {
    type Err = crate::Err;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)").unwrap();
        if let Some(c) = re.captures(input) {
            let i = Input {
                min: c.index(1).parse::<u32>().unwrap(),
                max: c.index(2).parse::<u32>().unwrap(),
                letter: c.index(3).parse::<char>().unwrap(),
                password: c.index(4).parse::<String>().unwrap(),
            };

            return Ok(i);
        }

        Err(Err::Failed())
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_sample() -> Result<(), Err> {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let passwords = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(first(&passwords)?, 2);

        Ok(())
    }

    #[test]
    fn day2_part2_sample() -> Result<(), Err> {
        let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let passwords = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&passwords)?, 1);

        Ok(())
    }
}
