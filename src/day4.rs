use crate::Err;
use regex::Regex;
use std::collections::HashMap;
use std::ops::{Add, Index};

pub fn first(input: &[String]) -> Result<u64, Err> {
    let passports = group(input);

    let mut valid_count: u64 = 0;
    for passport_input in passports {
        let mut passport: HashMap<&str, &str> = HashMap::new();
        let passport_elements: Vec<&str> = passport_input.split(' ').collect();
        for passport_element in passport_elements {
            let mut element = passport_element.split(':');
            let k = element.next();
            let v = element.last();
            passport.insert(k.unwrap(), v.unwrap());
        }

        if passport.get("byr").is_some()
            && passport.get("iyr").is_some()
            && passport.get("eyr").is_some()
            && passport.get("hgt").is_some()
            && passport.get("hcl").is_some()
            && passport.get("ecl").is_some()
            && passport.get("pid").is_some()
        {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    let passports = group(input);

    let mut valid_count: u64 = 0;
    for passport_input in passports {
        let mut passport: HashMap<&str, &str> = HashMap::new();
        let passport_elements: Vec<&str> = passport_input.split(' ').collect();
        for passport_element in passport_elements {
            let mut element = passport_element.split(':');
            let k = element.next();
            let v = element.last();
            passport.insert(k.unwrap(), v.unwrap());
        }

        if byr(passport.get("byr"))
            && iyr(passport.get("iyr"))
            && eyr(passport.get("eyr"))
            && hgt(passport.get("hgt"))
            && hcl(passport.get("hcl"))
            && ecl(passport.get("ecl"))
            && pid(passport.get("pid"))
        {
            valid_count += 1;
        }
    }

    Ok(valid_count)
}

fn pid(pid: Option<&&str>) -> bool {
    if let Some(number) = pid {
        if number.len() == 9 {
            return true;
        }
    }

    false
}

fn ecl(ecl: Option<&&str>) -> bool {
    if let Some(eye_color) = ecl {
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if colors.contains(eye_color) {
            return true;
        }
    }

    false
}

fn hcl(hcl: Option<&&str>) -> bool {
    if let Some(hex_color) = hcl {
        let re = Regex::new(r"^#([a-fA-F0-9]{6})$").unwrap();
        if let Some(c) = re.captures(hex_color) {
            if c.len() == 2 {
                return true;
            }
        }
    }

    false
}

fn hgt(hgt: Option<&&str>) -> bool {
    if let Some(height) = hgt {
        let re = Regex::new(r"^(\d+)(\w+)").unwrap();
        if let Some(c) = re.captures(height) {
            let unit = c.index(2);
            let measurement = c.index(1).parse::<u32>().unwrap();

            if unit == "cm" && measurement >= 150 && measurement <= 193 {
                return true;
            }

            if unit == "in" && measurement >= 59 && measurement <= 76 {
                return true;
            }
        }
    }

    false
}

fn eyr(eyr: Option<&&str>) -> bool {
    if let Some(year) = eyr {
        let valid_year = year.parse::<u32>();
        if let Ok(valid_year) = valid_year {
            if valid_year >= 2020 && valid_year <= 2030 {
                return true;
            }
        }
    }

    false
}

fn iyr(iyr: Option<&&str>) -> bool {
    if let Some(year) = iyr {
        let valid_year = year.parse::<u32>();
        if let Ok(valid_year) = valid_year {
            if valid_year >= 2010 && valid_year <= 2020 {
                return true;
            }
        }
    }

    false
}

fn byr(byr: Option<&&str>) -> bool {
    if let Some(year) = byr {
        let valid_year = year.parse::<u32>();
        if let Ok(valid_year) = valid_year {
            if valid_year >= 1920 && valid_year <= 2002 {
                return true;
            }
        }
    }

    false
}

fn group(input: &[String]) -> Vec<String> {
    let mut grouped = Vec::new();

    let mut g = String::new();
    let mut index: usize = 0;
    for line in input {
        if g.is_empty() {
            g = g.add(line.as_str());
        } else {
            g = g.add(" ");
            g = g.add(line.as_str());
        }

        if line.is_empty() {
            grouped.insert(index, g.trim_end().to_string().clone());

            index += 1;

            g.clear();
            continue;
        }
    }

    grouped
}
