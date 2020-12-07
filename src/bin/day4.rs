use std::collections::HashMap;

fn main() -> () {
    let passports = Passports::parse(INPUT);
    println!("{}", passports.count_valid());
    println!("{}", passports.count_actually_valid());
}

type Passport = HashMap<String, String>;

#[derive(Debug)]
struct Passports {
    passports: Vec<Passport>,
}

impl Passports {
    fn parse(s: &str) -> Self {
        let passports: Vec<Passport> = s
            .split("\n\n")
            .map(|s| {
                let mut m = HashMap::new();
                for field in s.split_whitespace() {
                    let split: Vec<_> = field.split(":").collect();
                    let key = split[0];
                    let value = split[1];
                    m.insert(key.to_owned(), value.to_owned());
                }
                m
            })
            .collect();
        Passports { passports }
    }

    fn count_valid(&self) -> usize {
        self.passports
            .iter()
            .filter(|p| {
                ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .all(|key| p.contains_key(key.to_owned()))
            })
            .count()
    }

    fn count_actually_valid(&self) -> usize {
        self.passports.iter().filter(|p| actually_valid(p)).count()
    }
}

fn actually_valid(p: &Passport) -> bool {
    let is_num_in_range = |key, min, max| {
        p.get(key)
            .map(|value| {
                value
                    .parse::<usize>()
                    .map(|value| value >= min && value <= max)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    };
    let byr_valid = is_num_in_range("byr", 1920, 2002);
    let iyr_valid = is_num_in_range("iyr", 2010, 2020);
    let eyr_valid = is_num_in_range("eyr", 2020, 2030);
    let hgt_valid = p
        .get("hgt")
        .map(|hgt| {
            let cm_valid = hgt.len() == 5
                && hgt.ends_with("cm")
                && hgt[..3]
                    .parse::<usize>()
                    .map(|n| n >= 150 && n <= 193)
                    .unwrap_or(false);
            let in_valid = hgt.len() == 4
                && hgt.ends_with("in")
                && hgt[..2]
                    .parse::<usize>()
                    .map(|n| n >= 59 && n <= 76)
                    .unwrap_or(false);
            cm_valid || in_valid
        })
        .unwrap_or(false);
    let hcl_valid = p
        .get("hcl")
        .map(|hcl| {
            hcl.len() == 7
                && hcl.starts_with("#")
                && hcl.chars().skip(1).all(|c| match c {
                    '0'..='9' | 'a'..='f' => true,
                    _ => false,
                })
        })
        .unwrap_or(false);
    let ecl_valid = p
        .get("ecl")
        .map(|ecl| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|v| ecl == v)
        })
        .unwrap_or(false);
    let pid_valid = p
        .get("pid")
        .map(|pid| pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or(false);
    byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
}

const INPUT: &'static str = include_str!("day4.txt");
