fn main() -> () {
    let mut adapters: Vec<usize> = INPUT
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{}", distribution_of_voltages(&mut adapters));
    println!("{}", distinct_adapter_arrangements(adapters));
}

fn distribution_of_voltages(adapters: &mut Vec<usize>) -> usize {
    adapters.sort_unstable();
    let mut one_sizers = 0;
    let mut three_sizers = 1; // always a three sizer at the end
    let mut voltage = 0;
    for adapter in adapters.iter() {
        match adapter - voltage {
            1 => one_sizers += 1,
            3 => three_sizers += 1,
            _ => {}
        }
        voltage = *adapter;
    }
    one_sizers * three_sizers
}

fn distinct_adapter_arrangements(adapters: Vec<usize>) -> usize {
    let mut steps = vec![vec![0; adapters.len()]; adapters.len()];
    for i in 0..3 {
        if adapters[i] <= 3 {
            steps[0][i] = 1;
        }
    }
    for step in 1..adapters.len() {
        for (i, adapter) in adapters.iter().enumerate() {
            for j in 1..=3 {
                if i >= j && adapter - adapters[i - j] <= 3 {
                    steps[step][i] += steps[step - 1][i - j]
                }
            }
        }
    }
    steps.iter().map(|step| step.last().unwrap_or(&0)).sum()
}

const INPUT: &'static str = include_str!("day10.txt");
