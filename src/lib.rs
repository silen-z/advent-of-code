mod combinators;

pub use combinators::*;

pub use anyhow::*;

pub struct NoSolution;

impl std::fmt::Display for NoSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "no solution found")
    }
}

pub fn split_once<'a>(s: &'a str, delimiter: &'_ str) -> Option<(&'a str, &'a str)> {
    let mid = s.find(delimiter)?;

    Some((&s[..mid], &s[mid + delimiter.len()..]))
}

use std::fmt::Display;
pub trait UserErrorMessageExit<T> {
    fn or_exit(self) -> T;
    fn or_exit_with(self, msg: impl Display) -> T;
}

impl<T, E: Display> UserErrorMessageExit<T> for Result<T, E> {
    fn or_exit(self) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    }

    fn or_exit_with(self, msg: impl Display) -> T {
        match self {
            Result::Ok(v) => v,
            Result::Err(err) => {
                eprintln!("{}: {}", msg, err);
                std::process::exit(0);
            }
        }
    }
}

impl<T> UserErrorMessageExit<T> for Option<T> {
    fn or_exit(self) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => {
                std::process::exit(0);
            }
        }
    }

    fn or_exit_with(self, msg: impl Display) -> T {
        match self {
            Option::Some(v) => v,
            Option::None => {
                eprintln!("{}", msg);
                std::process::exit(0);
            }
        }
    }
}

pub mod solutions {
    use std::{fmt::Display, path::Path};

    pub trait Solution {
        fn solve(&self) -> anyhow::Result<()>;
    }

    impl<Input, Part1, Out1, Part2, Out2> Solution for (Input, Part1, Part2)
    where
        Input: AsRef<Path>,
        Part1: Fn(&str) -> anyhow::Result<Option<Out1>>,
        Part2: Fn(&str, &Out1) -> anyhow::Result<Option<Out2>>,
        Out1: Display,
        Out2: Display,
    {
        fn solve(&self) -> anyhow::Result<()> {
            let (path, part1, part2) = self;
            let input = std::fs::read_to_string(path)?;

            let part1 = part1(&input)?.unwrap_or("no solution");
            let part2 = part2(&input, &part1)?.unwrap_or("no solution");

            println!("part1: {}\npart2: {}", part1, part2);

            Ok(())
        }
    }
}
