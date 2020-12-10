fn main() -> () {
    let boarding_passes = BoardingPasses::parse(INPUT);
    println!("{}", boarding_passes.highest_seat_id());
    println!("{}", boarding_passes.missing_seat_id());
}

#[derive(Debug)]
struct BoardingPass(String);

#[derive(Debug)]
struct BoardingPasses {
    passes: Vec<BoardingPass>,
}

impl BoardingPasses {
    fn parse(s: &str) -> Self {
        let passes: Vec<BoardingPass> = s.lines().map(|s| BoardingPass(s.to_owned())).collect();
        BoardingPasses { passes }
    }

    fn highest_seat_id(&self) -> usize {
        self.passes.iter().map(|p| p.seat_id()).max().unwrap_or(0)
    }

    fn missing_seat_id(&self) -> usize {
        let mut ids: Vec<usize> = self.passes.iter().map(|p| p.seat_id()).collect();
        ids.sort_unstable();
        for window in (&ids[..]).windows(2) {
            if window[1] - window[0] > 1 {
                return window[0] + 1;
            }
        }
        return 0;
    }
}

impl BoardingPass {
    fn row(&self) -> usize {
        binary_search('F', 'B', 128, self.0.chars().take(7))
    }

    fn seat(&self) -> usize {
        binary_search('L', 'R', 8, self.0.chars().skip(7).take(3))
    }

    fn seat_id(&self) -> usize {
        self.row() * 8 + self.seat()
    }
}

fn binary_search<T, Ts>(lower_half: T, upper_half: T, upper_bound: usize, ts: Ts) -> usize
where
    T: PartialEq,
    Ts: Iterator<Item = T>,
{
    // upper exclusive, lower inclusive
    let mut upper = upper_bound;
    let mut lower = 0;
    for t in ts {
        if t == lower_half {
            upper -= (upper - lower) / 2;
        }
        if t == upper_half {
            lower += (upper - lower) / 2;
        }
    }
    lower
}

#[test]
fn test_seat_id() {
    let pass = BoardingPass("FBFBBFFRLR".to_owned());
    assert_eq!(pass.row(), 44);
    assert_eq!(pass.seat(), 5);
    assert_eq!(pass.seat_id(), 357);
}

const INPUT: &'static str = include_str!("day05.txt");
