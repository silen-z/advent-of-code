fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/2020_05.txt")?;

    let ids = input.lines().map(decode_boarding_id);

    let result = match advent_of_code::part() {
        advent_of_code::Part::One => ids.max().unwrap(),
        advent_of_code::Part::Two => {
            let mut ids: Vec<u16> = ids.collect();
            ids.sort_unstable();
            ids.windows(2)
                .find_map(|pair| match pair {
                    [a, b] if a + 1 != *b => Some(a + 1),
                    _ => None,
                })
                .unwrap()
        }
    };

    println!("{}", result);

    Ok(())
}

fn decode_boarding_id(code: &str) -> u16 {
    // code.chars().fold(0, |a, c| match c {
    //     'F' | 'L' => a << 1,
    //     'B' | 'R' => (a << 1) | 1,
    //     _ => panic!(),
    // })

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
