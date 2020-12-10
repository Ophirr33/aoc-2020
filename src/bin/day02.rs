use lazy_static::lazy_static;
use regex::Regex;

fn main() -> () {
    println!(
        "{}",
        INPUT
            .lines()
            .map(PolicyAndPassword::parse)
            .filter(PolicyAndPassword::is_valid_old)
            .count()
    );
    println!(
        "{}",
        INPUT
            .lines()
            .map(PolicyAndPassword::parse)
            .filter(PolicyAndPassword::is_valid_new)
            .count()
    );
}

#[derive(Debug)]
struct PolicyAndPassword {
    letter: char,
    min: usize,
    max: usize,
    password: String,
}

impl PolicyAndPassword {
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+)-(\\d+) (\\w): (\\w+)").unwrap();
        }
        let caps = RE.captures(s).unwrap();
        let min = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let max = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let letter = caps.get(3).unwrap().as_str().chars().next().unwrap();
        let password = caps.get(4).unwrap().as_str().to_owned();
        PolicyAndPassword {
            min,
            max,
            letter,
            password,
        }
    }

    pub fn is_valid_old(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();
        count >= self.min && count <= self.max
    }

    pub fn is_valid_new(&self) -> bool {
        let first = self.password.chars().nth(self.min - 1).unwrap();
        let second = self.password.chars().nth(self.max - 1).unwrap();
        (first == self.letter) ^ (second == self.letter)
    }
}

const INPUT: &'static str = include_str!("day02.txt");
