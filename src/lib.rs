pub fn is_part2() -> bool {
    matches!(std::env::args().nth(1), Some(s) if s == "--part2")
}
