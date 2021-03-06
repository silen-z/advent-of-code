use advent_of_code::prelude::*;

const INPUT: &str = include_str!("../../input/2020_07.txt");

fn main() {
    let bags: BagRules = INPUT.parse().or_exit_with("can't parse input");

    let gold_id = bags
        .find_index("shiny gold")
        .or_exit_with("there is no shiny gold bag");

    let result = match advent_of_code::part() {
        advent_of_code::Part::One => bags.containers(gold_id),
        advent_of_code::Part::Two => bags.contained_bags(gold_id),
    };

    println!("{}", result);
}

#[derive(Default, Debug)]
struct BagRules {
    bags: Vec<String>,
    rules: Vec<Rule>,
}

impl BagRules {
    fn get_or_create(&mut self, bag: &str) -> usize {
        if let Some(index) = self.bags.iter().position(|b| b == bag) {
            index
        } else {
            let index = self.bags.len();
            self.bags.push(bag.to_owned());
            index
        }
    }

    fn new_rule(&mut self, container: usize, child: usize, amount: usize) {
        let rule = Rule {
            container,
            child,
            amount,
        };
        self.rules.push(rule);
    }

    fn find_index(&self, bag: &str) -> Option<usize> {
        self.bags.iter().position(|b| b == bag)
    }

    fn containers(&self, index: usize) -> usize {
        let mut visited = Vec::with_capacity(self.bags.len());

        self.count_containers(index, &mut visited) - 1
    }

    fn count_containers(&self, index: usize, visited: &mut Vec<usize>) -> usize {
        visited.push(index);
        let mut sum = 1;
        for edge in &self.rules {
            if edge.child == index && !visited.contains(&edge.container) {
                sum += self.count_containers(edge.container, visited);
            }
        }
        sum
    }

    fn contained_bags(&self, index: usize) -> usize {
        self.count_contained_bags(index) - 1
    }

    fn count_contained_bags(&self, index: usize) -> usize {
        self.rules
            .iter()
            .filter(|r| r.container == index)
            .fold(1, |sum, rule| {
                sum + rule.amount * self.count_contained_bags(rule.child)
            })
    }
}

#[derive(Debug)]
struct Rule {
    container: usize,
    child: usize,
    amount: usize,
}

impl std::str::FromStr for BagRules {
    type Err = InvalidFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bag_rules = BagRules::default();

        for (ln, line) in s.lines().enumerate() {
            let (container, rules) = split_once(line, " bags contain ")
                .ok_or_else(|| InvalidFormat(line.to_owned(), ln))?;

            let container = bag_rules.get_or_create(container);

            if rules != "no other bags." {
                for rule in rules.split(", ") {
                    let (amount, bag) =
                        split_once(rule, " ").ok_or_else(|| InvalidFormat(line.to_owned(), ln))?;

                    let (bag, _) = split_once(bag, " bag")
                        .ok_or_else(|| InvalidFormat(line.to_owned(), ln))?;

                    let amount = amount
                        .parse()
                        .map_err(|_| InvalidFormat(line.to_owned(), ln))?;
                    let bag = bag_rules.get_or_create(&bag);

                    bag_rules.new_rule(container, bag, amount);
                }
            }
        }

        Ok(bag_rules)
    }
}

#[derive(Debug)]
struct InvalidFormat(String, usize);

impl std::fmt::Display for InvalidFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid bag rule '{}' on line {}", self.0, self.1)
    }
}

impl std::error::Error for InvalidFormat {}
