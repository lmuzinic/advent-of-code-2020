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

mod tests {
    use super::*;

    #[test]
    fn day4_part1_sample() -> Result<(), Err> {
        let input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
            "",
        ];
        let passports = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(first(&passports)?, 2);

        Ok(())
    }

    #[test]
    fn day4_part2_sample_invalid() -> Result<(), Err> {
        let input = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
            "",
        ];
        let passports = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&passports)?, 0);

        Ok(())
    }

    #[test]
    fn day4_part2_sample_valid() -> Result<(), Err> {
        let input = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
            "",
        ];
        let passports = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(second(&passports)?, 4);

        Ok(())
    }

    #[test]
    fn invalid_byr() {
        assert!(!byr(Some(&"2003")))
    }

    #[test]
    fn valid_byr() {
        assert!(byr(Some(&"2002")))
    }

    #[test]
    fn valid_hgt_in() {
        assert!(hgt(Some(&"60in")))
    }

    #[test]
    fn valid_hgt_cm() {
        assert!(hgt(Some(&"190cm")))
    }

    #[test]
    fn invalid_hgt_in() {
        assert!(!hgt(Some(&"190in")))
    }

    #[test]
    fn invalid_hgt_cm() {
        assert!(!hgt(Some(&"190")))
    }

    #[test]
    fn valid_hcl() {
        assert!(hcl(Some(&"#123abc")))
    }

    #[test]
    fn invalid_hcl() {
        assert!(!hcl(Some(&"#123abcd")));
        assert!(!hcl(Some(&"123abc")));
    }

    #[test]
    fn valid_ecl() {
        assert!(ecl(Some(&"brn")))
    }

    #[test]
    fn invalid_ecl() {
        assert!(!ecl(Some(&"wat")))
    }

    #[test]
    fn valid_pid() {
        assert!(pid(Some(&"000000001")))
    }

    #[test]
    fn invalid_pid() {
        assert!(!pid(Some(&"0123456789")))
    }
}
