use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = File::open("input/2020_01.txt")?;
    let reader = BufReader::new(input_file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<_, _>>()?;

    let result =
        match advent_of_code::part() {
            advent_of_code::Part::One => numbers
                .combinations::<CombineTwice>()
                .find_map(|(n1, n2)| if n1 + n2 == 2020 { Some(n1 * n2) } else { None }),
            advent_of_code::Part::Two => {
                numbers
                    .combinations::<CombineThreeTimes>()
                    .find_map(|(n1, n2, n3)| match n1 + n2 + n3 == 2020 {
                        true => Some(n1 * n2 * n3),
                        false => None,
                    })
            }
        };

    if let Some(solution) = result {
        println!("{}", solution);
    } else {
        println!("{}", "no solution found");
    }

    Ok(())
}

struct Combinations<'s, E, A> {
    source: &'s [E],
    advancer: A,
}

trait Combiner<'s, E> {
    type Out;
    fn next(&mut self, source: &'s [E]) -> Option<Self::Out>;
}

impl<'s, E, A> Iterator for Combinations<'s, E, A>
where
    A: Combiner<'s, E>,
{
    type Item = A::Out;

    fn next(&mut self) -> Option<Self::Item> {
        self.advancer.next(self.source)
    }
}

struct CombineTwice(usize, usize);

impl<'s, E: 's> Combiner<'s, E> for CombineTwice {
    type Out = (&'s E, &'s E);
    fn next(&mut self, source: &'s [E]) -> Option<Self::Out> {
        match (source.get(self.0), source.get(self.1)) {
            (Some(t1), Some(t2)) => {
                if self.1 + 1 < source.len() {
                    self.1 += 1;
                } else {
                    self.0 += 1;
                    self.1 = self.0 + 1;
                }
                Some((t1, t2))
            }
            _ => None,
        }
    }
}

impl Default for CombineTwice {
    fn default() -> Self {
        CombineTwice(0, 1)
    }
}

struct CombineThreeTimes(usize, usize, usize);

impl<'s, E: 's> Combiner<'s, E> for CombineThreeTimes {
    type Out = (&'s E, &'s E, &'s E);
    fn next(&mut self, source: &'s [E]) -> Option<Self::Out> {
        match (source.get(self.0), source.get(self.1), source.get(self.2)) {
            (Some(t1), Some(t2), Some(t3)) => {
                self.2 += 1;

                if self.2 >= source.len() {
                    self.1 += 1;
                    self.2 = self.1 + 1;
                }

                if self.1 + 1 >= source.len() {
                    self.0 += 1;
                    self.1 = self.0 + 1;
                    self.2 = self.1 + 1;
                }
                Some((t1, t2, t3))
            }
            _ => None,
        }
    }
}

impl Default for CombineThreeTimes {
    fn default() -> Self {
        CombineThreeTimes(0, 1, 2)
    }
}

trait Combinable<'s, E> {
    fn combinations<A>(&self) -> Combinations<E, A>
    where
        A: Combiner<'s, E> + Default;
}

impl<'s, E> Combinable<'s, E> for [E] {
    fn combinations<A>(&self) -> Combinations<E, A>
    where
        A: Combiner<'s, E> + Default,
    {
        Combinations {
            source: self,
            advancer: A::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zip_each_iter() {
        let input = &[1, 2, 3];
        let mut iter = input.combinations::<CombineTwice>();

        assert_eq!(iter.next(), Some((&1, &2)));
        assert_eq!(iter.next(), Some((&1, &3)));
        assert_eq!(iter.next(), Some((&2, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn triple_zip_each_iter() {
        let input = &[1, 2, 3, 4, 5];
        let mut iter = input.combinations::<CombineThreeTimes>();

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
        let input = &[1721, 979, 366, 299, 675, 1456];

        let result = input
            .combinations::<CombineThreeTimes>()
            .find_map(|(n1, n2, n3)| {
                if n1 + n2 + n3 == 2020 {
                    Some(n1 * n2 * n3)
                } else {
                    None
                }
            });

        assert_eq!(result, Some(241861950));
    }
}
