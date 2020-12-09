use std::collections::{HashSet, VecDeque};

fn main() -> () {
    let xmas_cracker = XmasCracker::parse(INPUT);
    let invalid_num = xmas_cracker.find_invalid_num(25);
    println!("{:?}", invalid_num);
    if let Some(invalid_num) = invalid_num {
        let contiguious = xmas_cracker.find_contiguous_sum(invalid_num);
        println!("{:?}", contiguious);
    }
}

#[derive(Debug)]
struct XmasCracker {
    nums: Vec<usize>,
}

impl XmasCracker {
    fn parse(s: &str) -> Self {
        let nums = s
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        XmasCracker { nums }
    }

    fn find_invalid_num(&self, preamble_size: usize) -> Option<usize> {
        let mut window_set: HashSet<usize> = HashSet::new();
        let mut window_que: VecDeque<usize> = VecDeque::new();
        for &n in self.nums[0..preamble_size].iter() {
            window_set.insert(n);
            window_que.push_back(n);
        }
        for &n in self.nums[preamble_size..].iter() {
            if !window_que.iter().any(|&m| window_set.contains(&(n - m))) {
                return Some(n);
            }
            let m = window_que.pop_front().unwrap();
            window_que.push_back(n);

            let _ = window_set.remove(&m);
            window_set.insert(n);
        }
        None
    }

    fn find_contiguous_sum(&self, target_sum: usize) -> Option<usize> {
        let mut sum = 0;
        let mut window: VecDeque<usize> = VecDeque::new();
        for &n in self.nums.iter() {
            sum += n;
            window.push_back(n);
            while sum > target_sum && !window.is_empty() {
                let m = window.pop_front().unwrap();
                sum -= m;
            }
            if sum == target_sum && !window.is_empty() {
                return Some(window.iter().min().unwrap() + window.iter().max().unwrap());
            }
        }
        return None;
    }
}

const INPUT: &'static str = include_str!("day9.txt");
