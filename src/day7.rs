use crate::Err;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

pub fn first(input: &[String]) -> Result<u64, Err> {
    let collection = make_collection(input);

    let mut count: u64 = 0;
    for item in collection.keys() {
        if contains(item, "shiny gold", &collection) {
            count += 1;
        }
    }

    Ok(count)
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    let collection = make_collection(input);

    Ok(sum_bags("shiny gold", &collection) - 1)
}

fn make_collection(input: &[String]) -> HashMap<&str, Vec<Contents>> {
    let mut collection: HashMap<&str, Vec<Contents>> = HashMap::new();
    let re = Regex::new(r"^(\d+) (\w+ \w+)").unwrap();

    for bag_information in input {
        let info: Vec<&str> = bag_information.split(" bags contain ").collect();

        let bag = info[0];
        let mut contents_vec: Vec<Contents> = Vec::new();
        let contents: Vec<&str> = info[1].split(", ").collect();
        for sub in contents {
            if let Some(c) = re.captures(sub) {
                let contents = Contents::new(
                    c.index(1).parse::<usize>().unwrap(),
                    c.index(2).parse::<String>().unwrap(),
                );

                contents_vec.push(contents);
            }
        }

        collection.insert(bag, contents_vec);
    }
    collection
}

#[derive(Debug)]
struct Contents {
    pub q: usize,
    pub bag: String,
}

impl Contents {
    pub fn new(q: usize, bag: String) -> Self {
        Contents { q, bag }
    }
}

fn contains(input: &str, bag: &str, map: &HashMap<&str, Vec<Contents>>) -> bool {
    if let Some(contents) = map.get(input) {
        for content in contents {
            if content.bag == bag {
                return true;
            }

            if contains(&content.bag.as_str(), bag, &map) {
                return true;
            } else {
                continue;
            }
        }

        return false;
    }

    false
}

fn sum_bags(bag: &str, map: &HashMap<&str, Vec<Contents>>) -> u64 {
    if let Some(contents) = map.get(bag) {
        let mut num_bags: u64 = 1;
        for content in contents {
            num_bags += content.q as u64 * sum_bags(&content.bag, map);
        }

        if contents.is_empty() {
            return 1;
        }

        return num_bags as u64;
    }

    panic!();
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1_sample() -> Result<(), Err> {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        let bags = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(first(&bags)?, 4);

        Ok(())
    }

    #[test]
    fn day7_part2_sample() -> Result<(), Err> {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        let bags = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&bags)?, 32);

        Ok(())
    }

    #[test]
    fn day7_part2_another_sample() -> Result<(), Err> {
        let input = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        let bags = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&bags)?, 126);

        Ok(())
    }
}
