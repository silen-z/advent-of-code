use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/2020_04.txt");

fn main() {
    let check_passport = match advent_of_code::part() {
        advent_of_code::Part::One => PassportData::is_part1_valid,
        advent_of_code::Part::Two => PassportData::is_part2_valid,
    };

    let valid_passports = PassportData::from_list(INPUT)
        .filter(|p| check_passport(p))
        .count();

    println!("{}", valid_passports);
}

#[derive(Debug)]
struct PassportData(HashMap<String, String>);

impl PassportData {
    fn from_list<'i>(input: &'i str) -> impl Iterator<Item = PassportData> + 'i {
        input.split("\n\n").filter_map(|s| s.parse().ok())
    }

    fn is_part1_valid(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|(key, _)| self.0.contains_key(*key))
    }

    fn is_part2_valid(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|(key, validator)| matches!(self.0.get(*key), Some(value) if validator(value)))
    }
}

type FieldValidator = dyn Fn(&str) -> bool;

const REQUIRED_FIELDS: &[(&str, &FieldValidator)] = &[
    ("byr", &|byr| matches!(byr.parse::<u32>(), Ok(1920..=2002))),
    ("iyr", &|iyr| matches!(iyr.parse::<u32>(), Ok(2010..=2020))),
    ("eyr", &|eyr| matches!(eyr.parse::<u32>(), Ok(2020..=2030))),
    ("hgt", &is_valid_height),
    ("hcl", &is_valid_color),
    ("ecl", &|ecl| {
        matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }),
    ("pid", &|pid| pid.len() == 9 && pid.parse::<u32>().is_ok()),
];

fn is_valid_height(hgt: &str) -> bool {
    if let Some(cm) = hgt.strip_suffix("cm") {
        return matches!(cm.parse(), Ok(150..=193));
    }

    if let Some(inches) = hgt.strip_suffix("in") {
        return matches!(inches.parse(), Ok(59..=76));
    }

    false
}

fn is_valid_color(s: &str) -> bool {
    match s.strip_prefix('#') {
        Some(code) => code.len() == 6 && code.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9')),
        _ => false,
    }
}

impl std::str::FromStr for PassportData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = HashMap::new();

        for pair in s.split_whitespace() {
            let mut parts = pair.split(':');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => {
                    values.insert(key.to_string(), value.to_string());
                }
                _ => return Err(format!("invalid passport: {:?}", s)),
            }
        }

        Ok(PassportData(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let valid_passports = PassportData::from_list(input)
            .filter(|p| p.is_part1_valid())
            .count();

        assert_eq!(valid_passports, 2);
    }
}
