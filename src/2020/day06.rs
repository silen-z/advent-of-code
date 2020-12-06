use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/2020_06.txt")?;

    let strategy = match advent_of_code::part() {
        advent_of_code::Part::One => anyone_answered_yes,
        advent_of_code::Part::Two => everyone_answered_yes,
    };

    let result: usize = input.split("\n\n").map(strategy).sum();

    println!("{}", result);

    Ok(())
}

fn anyone_answered_yes(group: &str) -> usize {
    let answers: HashSet<char> = group.chars().filter(|c| matches!(c, 'a'..='z')).collect();
    answers.len()
}

fn everyone_answered_yes(group: &str) -> usize {
    let mut lines = group.lines();

    let answers: HashSet<char> = match lines.next() {
        Some(first) => first.chars().collect(),
        _ => return 0,
    };

    lines
        .fold(answers, |answers, line| {
            let other_answers = line.chars().collect();
            answers.intersection(&other_answers).copied().collect()
        })
        .len()
}
