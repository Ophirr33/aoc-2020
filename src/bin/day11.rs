use std::convert::TryFrom;

fn main() -> () {
    let seats = AirportSeats::parse(INPUT);
    let seats = advance_until_stable(seats);
    println!("{}", seats.count_occupied());
}

fn advance_until_stable(mut seats: AirportSeats) -> AirportSeats {
    loop {
        let new_seats = seats.advance_seats();
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

    fn advance_seats(&self) -> Self {
        let mut result = self.clone();
        for (y, row) in self.seats.iter().enumerate() {
            for (x, spot) in row.iter().enumerate() {
                let num_occupied = self.occupied_seats_by(x, y);
                match spot {
                    Spot::Empty if num_occupied == 0 => result.seats[y][x] = Spot::Occupied,
                    Spot::Occupied if num_occupied >= 4 => result.seats[y][x] = Spot::Empty,
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
