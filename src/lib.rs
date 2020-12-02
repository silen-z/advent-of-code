pub enum Part {
    One,
    Two,
}

pub fn part() -> Part {
    match std::env::args().nth(1) {
        Some(s) if s == "--part2" => Part::Two,
        _ => Part::One,
    }
}
