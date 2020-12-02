use std::io::{BufRead, BufReader};
use std::{fs::File, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("input/2020_02.txt")?;
    let reader = BufReader::new(input_file);

    let check_policy = match advent_of_code::part() {
        advent_of_code::Part::One => Policy::is_part1_valid,
        advent_of_code::Part::Two => Policy::is_part2_valid,
    };

    let solution = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .filter(check_policy)
        .count();

    println!("{}", solution);

    Ok(())
}

#[derive(Debug)]
struct Policy {
    range: RangeInclusive<usize>,
    password: String,
    c: char,
}

impl Policy {
    fn is_part1_valid(&self) -> bool {
        let actual = self.password.matches(self.c).count();
        self.range.contains(&actual)
    }

    fn is_part2_valid(&self) -> bool {
        let c: Vec<char> = self.password.chars().collect();

        let first = self.range.start() - 1;
        let second = self.range.end() - 1;

        matches!((
            c.get(first), c.get(second)),
            (Some(c1), Some(c2)) if (*c1 == self.c) ^ (*c2 == self.c)
        )
    }
}

impl std::str::FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match parts.as_slice() {
            [range, c, password] => {
                let parts: Vec<&str> = range.split('-').collect();
                let range = match parts.as_slice() {
                    [from, to] => {
                        let from = from.parse().map_err(|_| "invalid from number")?;
                        let to = to.parse().map_err(|_| "invalid to number")?;
                        Ok(from..=to)
                    }
                    _ => Err("invalid range"),
                }?;

                Ok(Policy {
                    range,
                    c: c.chars().next().unwrap(),
                    password: (*password).to_owned(),
                })
            }
            _ => Err("invalid format"),
        }
    }
}
