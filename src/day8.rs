use crate::Err;

pub fn first(input: &[String]) -> Result<u64, Err> {
    let mut commands = parse(input);

    let mut current: usize = 0;
    let mut accumulator: i64 = 0;

    loop {
        match commands[current] {
            Ops::Nop(_) => {
                commands[current] = Ops::Exit;
                current += 1;
            }
            Ops::Jmp(v) => {
                commands[current] = Ops::Exit;
                if v > 0 {
                    current += v.abs() as usize;
                } else {
                    current -= v.abs() as usize;
                }
            }
            Ops::Acc(v) => {
                commands[current] = Ops::Exit;
                accumulator += v;
                current += 1;
            }
            Ops::Exit => return Ok(accumulator as u64),
        }
    }
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    for i in 0..input.len() {
        let commands = parse_fixed(input, i);

        let mut current: usize = 0;
        let mut accumulator: i64 = 0;
        let mut infinite: usize = 0;

        loop {
            match commands[current] {
                Ops::Nop(_) => {
                    current += 1;
                }
                Ops::Jmp(v) => {
                    if v > 0 {
                        current += v.abs() as usize;
                    } else {
                        current -= v.abs() as usize;
                    }
                }
                Ops::Acc(v) => {
                    accumulator += v;
                    current += 1;
                }
                Ops::Exit => {
                    if accumulator > 0 {
                        return Ok(accumulator as u64);
                    }
                }
            }

            if infinite > input.len() {
                break;
            } else {
                infinite += 1;
            }
        }
    }

    Err(Err::Failed())
}

fn parse(input: &[String]) -> Vec<Ops> {
    let mut commands: Vec<Ops> = Vec::new();

    for c in input {
        let command: Vec<&str> = c.split_whitespace().collect();

        match command[0] {
            "nop" => {
                commands.push(Ops::Nop(command[1].parse::<i64>().unwrap()));
            }
            "acc" => {
                commands.push(Ops::Acc(command[1].parse::<i64>().unwrap()));
            }
            "jmp" => {
                commands.push(Ops::Jmp(command[1].parse::<i64>().unwrap()));
            }
            &_ => {
                panic!();
            }
        }
    }

    commands
}

fn parse_fixed(input: &[String], fix: usize) -> Vec<Ops> {
    let mut commands: Vec<Ops> = Vec::new();

    for (pos, c) in input.iter().enumerate() {
        let command: Vec<&str> = c.split_whitespace().collect();

        match command[0] {
            "nop" => {
                if pos == fix {
                    commands.push(Ops::Jmp(command[1].parse::<i64>().unwrap()));
                } else {
                    commands.push(Ops::Nop(command[1].parse::<i64>().unwrap()));
                }
            }
            "acc" => {
                if pos == fix {
                    let mut empty_commands = Vec::new();
                    empty_commands.push(Ops::Exit);

                    return empty_commands;
                } else {
                    commands.push(Ops::Acc(command[1].parse::<i64>().unwrap()));
                }
            }
            "jmp" => {
                if pos == fix {
                    commands.push(Ops::Nop(command[1].parse::<i64>().unwrap()));
                } else {
                    commands.push(Ops::Jmp(command[1].parse::<i64>().unwrap()));
                }
            }
            &_ => {
                panic!();
            }
        }
    }

    commands.push(Ops::Exit);

    commands
}

#[derive(Debug)]
enum Ops {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    Exit,
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1_sample() -> Result<(), Err> {
        let input = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ];
        let instructions = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(first(&instructions)?, 5);

        Ok(())
    }

    #[test]
    fn day8_part2_sample() -> Result<(), Err> {
        let input = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ];
        let instructions = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&instructions)?, 8);

        Ok(())
    }
}
