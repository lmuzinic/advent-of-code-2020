use crate::Err;
use std::collections::HashMap;

pub fn first(input: &[String]) -> Result<u64, Err> {
    let groups = group(input);

    let mut uniques_count: usize = 0;
    for group in groups {
        let mut group_answers = Vec::new();
        for person in group {
            let answers = person.chars();
            for answer in answers {
                if !group_answers.contains(&answer) {
                    group_answers.push(answer)
                }
            }
        }

        uniques_count += group_answers.len();
        group_answers.clear();
    }

    Ok(uniques_count as u64)
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    let groups = group(input);

    let mut uniques_count: usize = 0;
    for group in groups {
        let mut group_uniques: HashMap<char, usize> = HashMap::new();
        for person in &group {
            let answers = person.chars();
            for answer in answers {
                let count = group_uniques.entry(answer).or_insert(0);
                *count += 1;
            }
        }

        let mut group_common_answers: usize = 0;
        for value in group_uniques.values() {
            if *value == group.len() {
                group_common_answers += 1;
            }
        }

        uniques_count += group_common_answers;
    }

    Ok(uniques_count as u64)
}

fn group(input: &[String]) -> Vec<Vec<String>> {
    let mut all: Vec<Vec<String>> = Vec::new();

    let mut group: Vec<String> = Vec::new();
    for line in input {
        if line.is_empty() {
            all.push(group.clone());
            group.clear();
            continue;
        }

        group.push(line.clone());
    }

    all
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1_sample() -> Result<(), Err> {
        let input = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
        ];
        let groups = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(first(&groups)?, 11);

        Ok(())
    }

    #[test]
    fn day6_part2_sample() -> Result<(), Err> {
        let input = vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b", "",
        ];
        let groups = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&groups)?, 6);

        Ok(())
    }
}
