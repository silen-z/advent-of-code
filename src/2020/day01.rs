use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("input/2020_01.txt")?;
    let reader = BufReader::new(input_file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<i32>, _>>()?;

    let result = if advent_of_code::is_part2() {
        triple_zip_each(&numbers)
            .find_map(|(n1, n2, n3)| {
                if n1 + n2 + n3 == 2020 {
                    Some(n1 * n2 * n3)
                } else {
                    None
                }
            })
            .unwrap()
    } else {
        zip_each(&numbers)
            .find_map(|(n1, n2)| if n1 + n2 == 2020 { Some(n1 * n2) } else { None })
            .unwrap()
    };

    println!("{}", result);

    Ok(())
}

struct ZipEachIter<'a, T> {
    source: &'a [T],
    pos_1: usize,
    pos_2: usize,
}

fn zip_each<T>(source: &[T]) -> ZipEachIter<'_, T> {
    ZipEachIter {
        source,
        pos_1: 0,
        pos_2: 1,
    }
}

impl<'a, T> Iterator for ZipEachIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.source.get(self.pos_1), self.source.get(self.pos_2)) {
            (Some(t1), Some(t2)) => {
                if self.pos_2 + 1 < self.source.len() {
                    self.pos_2 += 1;
                } else {
                    self.pos_1 += 1;
                    self.pos_2 = self.pos_1 + 1;
                }
                Some((t1, t2))
            }
            _ => None,
        }
    }
}

struct TripleZipEachIter<'a, T> {
    source: &'a [T],
    pos_1: usize,
    pos_2: usize,
    pos_3: usize,
}

fn triple_zip_each<T>(source: &[T]) -> TripleZipEachIter<'_, T> {
    TripleZipEachIter {
        source,
        pos_1: 0,
        pos_2: 1,
        pos_3: 2,
    }
}

// 1 1 1
// 2 2 2
// 3 3 3
// 4 4 4
// 5 5 5

impl<'a, T> Iterator for TripleZipEachIter<'a, T> {
    type Item = (&'a T, &'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match (
            self.source.get(self.pos_1),
            self.source.get(self.pos_2),
            self.source.get(self.pos_3),
        ) {
            (Some(t1), Some(t2), Some(t3)) => {
                self.pos_3 += 1;

                if self.pos_3 >= self.source.len() {
                    self.pos_2 += 1;
                    self.pos_3 = self.pos_2 + 1;
                }

                if self.pos_2 + 1 >= self.source.len() {
                    self.pos_1 += 1;
                    self.pos_2 = self.pos_1 + 1;
                    self.pos_3 = self.pos_2 + 1;
                }
                Some((t1, t2, t3))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zip_each_iter() {
        let input = vec![1, 2, 3];
        let mut iter = zip_each(&input);

        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&1, &3)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn triple_zip_each_iter() {
        let input = vec![1, 2, 3, 4, 5];
        let mut iter = triple_zip_each(&input);

        assert_eq!(iter.next(), Some((&1, &2, &3)));
        assert_eq!(iter.next(), Some((&1, &2, &4)));
        assert_eq!(iter.next(), Some((&1, &2, &5)));
        assert_eq!(iter.next(), Some((&1, &3, &4)));
        assert_eq!(iter.next(), Some((&1, &3, &5)));
        assert_eq!(iter.next(), Some((&1, &4, &5)));
        assert_eq!(iter.next(), Some((&2, &3, &4)));
        assert_eq!(iter.next(), Some((&2, &3, &5)));
        assert_eq!(iter.next(), Some((&2, &4, &5)));
        assert_eq!(iter.next(), Some((&3, &4, &5)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn sample_input() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let result = triple_zip_each(&input).find_map(|(n1, n2, n3)| {
            if n1 + n2 + n3 == 2020 {
                Some(n1 * n2 * n3)
            } else {
                None
            }
        });

        assert_eq!(result, Some(241861950));
    }
}
