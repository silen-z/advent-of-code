use std::io::{BufRead, BufReader};
use std::{fs::File, ops::RangeInclusive};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("input/2020_02.txt")?;
    let reader = BufReader::new(input_file);

    let check_policy = match advent_of_code::is_part2() {
        true => Policy::is_part2_valid,
        false => Policy::is_part1_valid,
    };

    let result = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .filter(check_policy)
        .count();

    println!("{}", result);

    Ok(())
}

#[derive(Debug)]
struct Policy {
    expected: RangeInclusive<usize>,
    password: String,
    required_char: char,
}

impl Policy {
    fn is_part1_valid(&self) -> bool {
        let actual = self
            .password
            .chars()
            .filter(|c| *c == self.required_char)
            .count();
        self.expected.contains(&actual)
    }

    fn is_part2_valid(&self) -> bool {
        let c: Vec<char> = self.password.chars().collect();

        match (
            c.get(*self.expected.start() - 1),
            c.get(*self.expected.end() - 1),
        ) {
            (Some(c1), Some(c2))
                if (*c1 == self.required_char && *c2 != self.required_char)
                    || (*c1 != self.required_char && *c2 == self.required_char) =>
            {
                true
            }
            _ => false,
        }
    }
}

impl std::str::FromStr for Policy {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match parts.as_slice() {
            [range, c, password] => {
                let expected = match range.split('-').collect::<Vec<_>>().as_slice() {
                    [from, to] => {
                        let from = from.parse().map_err(|_| "invalid from number")?;
                        let to = to.parse().map_err(|_| "invalid to number")?;
                        Ok(from..=to)
                    }
                    _ => Err("invalid range"),
                }?;

                Ok(Policy {
                    expected,
                    required_char: c.chars().nth(0).unwrap(),
                    password: (*password).to_owned(),
                })
            }
            _ => Err("invalid format"),
        }
    }
}
