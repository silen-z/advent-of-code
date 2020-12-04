fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/2020_03.txt")?;

    let map: Map = input.parse()?;

    let trees = match advent_of_code::part() {
        advent_of_code::Part::One => map.slope(3, 1).count_trees(),
        advent_of_code::Part::Two => {
            let slopes = &mut [
                map.slope(1, 1),
                map.slope(3, 1),
                map.slope(5, 1),
                map.slope(7, 1),
                map.slope(1, 2),
            ];
            slopes
                .into_iter()
                .map(|s| s.count_trees())
                .fold(1, |a, c| a * c)
        }
    };

    println!("{}", trees);

    Ok(())
}

enum Tile {
    Empty,
    Tree,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
}

impl Map {
    fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        self.tiles.get(y * self.width + (x % self.width))
    }

    fn slope(&self, dir_x: usize, dir_y: usize) -> Slope {
        Slope {
            map: self,
            dir: (dir_x, dir_y),
            pos: (0, 0),
        }
    }
}

struct Slope<'map> {
    dir: (usize, usize),
    pos: (usize, usize),
    map: &'map Map,
}

impl Slope<'_> {
    fn count_trees(&mut self) -> usize {
        self.filter(|t| match t {
            Tile::Tree => true,
            _ => false,
        })
        .count()
    }
}

impl<'map> Iterator for Slope<'map> {
    type Item = &'map Tile;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos = (self.pos.0 + self.dir.0, self.pos.1 + self.dir.1);
        self.map.get(self.pos)
    }
}

impl std::str::FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or_else(|| s.len());

        let mut tiles = Vec::with_capacity(s.len());

        for (line_number, line) in s.lines().enumerate() {
            if line.len() > width {
                return Err(format!("line {} has different length", line_number + 1));
            }

            for (cn, c) in line.char_indices() {
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Tree,
                    _ => {
                        return Err(format!(
                            "invalid symbol at line {}:{}",
                            line_number + 1,
                            cn + 1
                        ))
                    }
                };

                tiles.push(tile)
            }
        }

        Ok(Map { tiles, width })
    }
}
