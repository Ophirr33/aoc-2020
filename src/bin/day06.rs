use std::collections::{HashMap, HashSet};
fn main() -> () {
    let part_1_answer: usize = INPUT
        .split("\n\n")
        .map(|s| {
            let mut set = HashSet::new();
            s.chars().filter(|c| !c.is_whitespace()).for_each(|c| {
                set.insert(c);
            });
            set.len()
        })
        .sum();
    println!("{}", part_1_answer);

    let part_2_answer: usize = INPUT
        .split("\n\n")
        .map(|s| {
            let mut map = HashMap::<char, usize>::new();
            s.chars().filter(|c| !c.is_whitespace()).for_each(|c| {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            });
            let num_lines = s.chars().filter(|&c| c == '\n').count() + 1;
            map.into_iter().filter(|(_, v)| *v == num_lines).count()
        })
        .sum();
    println!("{}", part_2_answer);
}

const INPUT: &'static str = include_str!("day06.txt");
