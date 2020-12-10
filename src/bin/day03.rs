fn main() -> () {
    let forest = Forest::parse(INPUT);
    println!("{:?}", forest);
    let inputs: [(usize, usize); 5] = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let answer: usize = inputs
        .iter()
        .map(|(down, right)| forest.num_trees(*down, *right))
        .product();
    println!("{}", answer);
}

#[derive(Debug)]
struct Forest {
    rows: Vec<Vec<bool>>,
}

impl Forest {
    fn parse(s: &str) -> Self {
        let rows = s
            .lines()
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect();
        Forest { rows }
    }

    fn num_trees(&self, down: usize, right: usize) -> usize {
        self.rows
            .iter()
            .step_by(down)
            .enumerate()
            .map(|(i, row)| row[(i * right) % row.len()] as usize)
            .sum()
    }
}

const INPUT: &'static str = include_str!("day03.txt");
