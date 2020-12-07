use std::collections::{HashMap, HashSet};

fn main() -> () {
    let bags = Bags::parse(INPUT);
    println!("{}", bags.count_outer_for_bag("shiny", "gold"));
    println!("{}", bags.count_inner_for_bag("shiny", "gold"));
}

type AdjectiveIndex = usize;
type ColorIndex = usize;
type BagHandle = (AdjectiveIndex, ColorIndex);

#[derive(Debug)]
struct Bag {
    adj: AdjectiveIndex,
    color: ColorIndex,
    contains: Vec<BagCapacity>,
    contained_by: Vec<BagHandle>,
}

impl Bag {
    fn handle(&self) -> BagHandle {
        (self.adj, self.color)
    }
}

#[derive(Debug)]
struct BagCapacity {
    amount: usize,
    handle: BagHandle,
}

#[derive(Debug, Default)]
struct Bags {
    colors: IndexedStrings,
    adjectives: IndexedStrings,
    bags: HashMap<BagHandle, Bag>,
}

impl Bags {
    fn parse(s: &str) -> Self {
        let mut bags = Bags::default();
        for line in s.lines() {
            let mut split = line.split(" ");
            let adj = bags.adjectives.get_or_add(split.next().unwrap());
            let color = bags.colors.get_or_add(split.next().unwrap());
            let _ = split.next();
            let _ = split.next();
            let mut contains = Vec::new();
            loop {
                let amount = split.next().unwrap();
                if amount == "no" {
                    break;
                }
                let amount = amount.parse::<usize>().unwrap();
                let adj = bags.adjectives.get_or_add(split.next().unwrap());
                let color = bags.colors.get_or_add(split.next().unwrap());
                contains.push(BagCapacity {
                    amount,
                    handle: (adj, color),
                });
                if split.next().unwrap().ends_with(".") {
                    break;
                }
            }
            bags.add_bag(Bag {
                adj,
                color,
                contains,
                contained_by: Vec::new(),
            });
        }
        bags
    }

    fn count_outer_for_bag(&self, adj: &str, color: &str) -> usize {
        let handle = (self.adjectives.get_i(adj), self.colors.get_i(color));
        let mut stack = self.bags.get(&handle).unwrap().contained_by.clone();
        let mut seen = HashSet::new();
        while !stack.is_empty() {
            let handle = stack.pop().unwrap();
            if seen.contains(&handle) {
                continue;
            } else {
                seen.insert(handle);
            }
            let b = self.bags.get(&handle).unwrap();
            b.contained_by.iter().for_each(|h| stack.push(*h));
        }
        seen.len()
    }

    fn count_inner_for_bag(&self, adj: &str, color: &str) -> usize {
        let handle = (self.adjectives.get_i(adj), self.colors.get_i(color));
        let mut stack = vec![(1, handle)];
        let mut sum = 0;
        while !stack.is_empty() {
            let (multiplier, handle) = stack.pop().unwrap();
            let b = self.bags.get(&handle).unwrap();
            b.contains.iter().for_each(|cap| {
                sum += multiplier * cap.amount;
                stack.push((multiplier * cap.amount, cap.handle));
            });
        }
        sum
    }

    #[allow(dead_code)]
    fn show_handle(&self, handle: BagHandle) -> String {
        format!(
            "{} {} bag",
            self.adjectives.get_s(handle.0),
            self.colors.get_s(handle.1)
        )
    }

    #[allow(dead_code)]
    fn show_bag(&self, handle: BagHandle) -> String {
        format!(
            "{} {} bag contains {}, and is contained by {}.",
            self.adjectives.get_s(handle.0),
            self.colors.get_s(handle.1),
            self.bags
                .get(&handle)
                .unwrap()
                .contains
                .iter()
                .map(|cap| { format!("{} {}", cap.amount, self.show_handle(cap.handle)) })
                .collect::<Vec<String>>()
                .join(", "),
            self.bags
                .get(&handle)
                .unwrap()
                .contained_by
                .iter()
                .map(|handle| self.show_handle(*handle))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    fn add_bag(&mut self, bag: Bag) {
        let handle = bag.handle();
        for contained in bag.contains.iter() {
            let (adj, color) = contained.handle;
            match self.bags.get_mut(&contained.handle) {
                None => {
                    self.bags.insert(
                        contained.handle.clone(),
                        Bag {
                            adj,
                            color,
                            contains: Vec::new(),
                            contained_by: vec![handle.clone()],
                        },
                    );
                }
                Some(b) => {
                    b.contained_by.push(handle.clone());
                }
            }
        }
        match self.bags.get_mut(&handle) {
            None => {
                self.bags.insert(handle, bag);
            }
            Some(b) => {
                b.contains = bag.contains;
            }
        }
    }
}

#[derive(Debug, Default)]
struct IndexedStrings {
    idx_to_string: Vec<String>,
    string_to_idx: HashMap<String, usize>,
}

impl IndexedStrings {
    fn get_or_add(&mut self, new: &str) -> usize {
        if let Some(u) = self.string_to_idx.get(new) {
            return *u;
        }
        let idx = self.idx_to_string.len();
        self.idx_to_string.push(new.to_owned());
        self.string_to_idx.insert(new.to_owned(), idx);
        idx
    }

    fn get_s(&self, i: usize) -> String {
        self.idx_to_string[i].clone()
    }

    fn get_i(&self, target: &str) -> usize {
        *self.string_to_idx.get(target).unwrap()
    }
}

const INPUT: &'static str = include_str!("day7.txt");
