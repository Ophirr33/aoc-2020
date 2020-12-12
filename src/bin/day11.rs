use std::convert::TryFrom;

fn main() -> () {
    let seats = AirportSeats::parse(INPUT);
    let part1_seats = advance_until_stable(seats.clone(), Strategy::Part1);
    println!("{}", part1_seats.count_occupied());
    let part2_seats = advance_until_stable(seats, Strategy::Part2);
    println!("{}", part2_seats.count_occupied());
}

fn advance_until_stable(mut seats: AirportSeats, strategy: Strategy) -> AirportSeats {
    loop {
        let new_seats = seats.advance_seats(strategy);
        if new_seats == seats {
            return new_seats;
        }
        seats = new_seats;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct AirportSeats {
    seats: Vec<Vec<Spot>>,
}

impl AirportSeats {
    fn parse(s: &str) -> Self {
        let seats = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(Spot::try_from)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        AirportSeats { seats }
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|&s| *s == Spot::Occupied).count())
            .sum()
    }

    fn advance_seats(&self, counting_strategy: Strategy) -> Self {
        let mut result = self.clone();
        let occupied_rule = match counting_strategy {
            Strategy::Part1 => 4,
            Strategy::Part2 => 5,
        };
        for (y, row) in self.seats.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                let num_occupied = match counting_strategy {
                    Strategy::Part1 => self.occupied_seats_by(x, y),
                    Strategy::Part2 => self.visible_occupied_seats_from(x, y),
                };
                match spot {
                    Spot::Empty if num_occupied == 0 => result.seats[y][x] = Spot::Occupied,
                    Spot::Occupied if num_occupied >= occupied_rule => {
                        result.seats[y][x] = Spot::Empty
                    }
                    _ => {}
                }
            }
        }
        result
    }

    fn occupied_seats_by(&self, x: usize, y: usize) -> usize {
        let mut result = 0;
        let mut count_if_occupied = |x, y| {
            if let Some(Spot::Occupied) = self.seats.get(y).and_then(|row: &Vec<Spot>| row.get(x)) {
                result += 1;
            }
        };
        count_if_occupied(x, y + 1);
        count_if_occupied(x + 1, y + 1);
        count_if_occupied(x + 1, y);
        if y > 0 {
            count_if_occupied(x + 1, y - 1);
            count_if_occupied(x, y - 1);
        }
        if x > 0 {
            count_if_occupied(x - 1, y);
            count_if_occupied(x - 1, y + 1);
        }
        if x > 0 && y > 0 {
            count_if_occupied(x - 1, y - 1);
        }
        result
    }

    fn visible_occupied_seats_from(&self, x: usize, y: usize) -> usize {
        use Op::*;
        let x_len = self.seats[0].len();
        let y_len = self.seats.len();
        let has_visible_from = |x_op: Op, y_op: Op| {
            let mut x = x;
            let mut y = y;
            loop {
                match x_op {
                    Add(1) if x == x_len - 1 => break,
                    Sub(1) if x == 0 => break,
                    _ => x = x_op.op(x),
                }
                match y_op {
                    Add(1) if y == y_len - 1 => break,
                    Sub(1) if y == 0 => break,
                    _ => y = y_op.op(y),
                }
                match self.seats[y][x] {
                    Spot::Occupied => return 1,
                    Spot::Empty => return 0,
                    _ => {}
                }
            }
            0
        };
        has_visible_from(Sub(1), Sub(1))
            + has_visible_from(Sub(1), Add(0))
            + has_visible_from(Sub(1), Add(1))
            + has_visible_from(Add(0), Add(1))
            + has_visible_from(Add(1), Add(1))
            + has_visible_from(Add(1), Add(0))
            + has_visible_from(Add(1), Sub(1))
            + has_visible_from(Add(0), Sub(1))
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(usize),
    Sub(usize),
}

impl Op {
    fn op(&self, n: usize) -> usize {
        match self {
            Op::Add(i) => n + i,
            Op::Sub(i) => n - i,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Strategy {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spot {
    Floor,
    Occupied,
    Empty,
}

impl TryFrom<char> for Spot {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spot::Floor),
            'L' => Ok(Spot::Empty),
            '#' => Ok(Spot::Occupied),
            bad => Err(bad),
        }
    }
}

const INPUT: &'static str = include_str!("day11.txt");
