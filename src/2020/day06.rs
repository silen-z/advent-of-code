const INPUT: &str = include_str!("../../input/2020_06.txt");

fn main() {
    let strategy = match advent_of_code::part() {
        advent_of_code::Part::One => anyone_answered_yes,
        advent_of_code::Part::Two => everyone_answered_yes,
    };

    let result: u32 = INPUT.split("\n\n").map(strategy).sum();

    println!("{}", result);
}

const OFFSET: u32 = 'a' as u32;

fn anyone_answered_yes(group: &str) -> u32 {
    group
        .chars()
        .filter(|c| *c != '\n')
        .fold(0u32, |bitset, c| bitset | (1 << (c as u32) - OFFSET))
        .count_ones()
}

fn everyone_answered_yes(group: &str) -> u32 {
    let mut set = u32::MAX;

    for line in group.lines() {
        let g = line
            .chars()
            .fold(0u32, |bitset, c| bitset | (1 << (c as u32) - OFFSET));

        set &= g;
    }

    set.count_ones()
}
