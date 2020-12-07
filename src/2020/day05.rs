use advent_of_code::prelude::*;

const INPUT: &str = include_str!("../../input/2020_05.txt");

fn main() {
    let ids = INPUT.lines().map(decode_boarding_id);

    let result = match advent_of_code::part() {
        advent_of_code::Part::One => ids.max().or_exit_with("empty input"),
        advent_of_code::Part::Two => {
            let mut ids: Vec<u16> = ids.collect();
            ids.sort_unstable();
            ids.windows(2)
                .find_map(|pair| match pair {
                    [a, b] if a + 1 != *b => Some(a + 1),
                    _ => None,
                })
                .or_exit_with("no solution found")
        }
    };

    println!("{}", result);
}

fn decode_boarding_id(code: &str) -> u16 {
    let (row, col) = code.split_at(7);

    let row = row.chars().fold(0, |a, c| match c {
        'F' => a << 1,
        'B' => (a << 1) | 1,
        _ => panic!(),
    });

    let col = col.chars().fold(0, |a, c| match c {
        'L' => a << 1,
        'R' => (a << 1) | 1,
        _ => panic!(),
    });

    (row << 3) | col
}
