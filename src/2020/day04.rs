use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input/2020_04.txt")?;

    let check_passport = match advent_of_code::part() {
        advent_of_code::Part::One => PassportData::is_part1_valid,
        advent_of_code::Part::Two => PassportData::is_part2_valid,
    };

    let valid_passports = PassportData::from_list(&input)
        .filter(|p| check_passport(p))
        .count();

    println!("{}", valid_passports);

    Ok(())
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
            .all(|(key, validator)| match self.0.get(*key) {
                Some(value) if validator(value) => true,
                _ => false,
            })
    }
}

const REQUIRED_FIELDS: &[(&str, &dyn Fn(&str) -> bool)] = &[
    ("byr", &|byr| match byr.parse::<u32>() {
        Ok(1920..=2002) => true,
        _ => false,
    }),
    ("iyr", &|iyr| match iyr.parse::<u32>() {
        Ok(2010..=2020) => true,
        _ => false,
    }),
    ("eyr", &|eyr| match eyr.parse::<u32>() {
        Ok(2020..=2030) => true,
        _ => false,
    }),
    ("hgt", &is_valid_height),
    ("hcl", &is_valid_color),
    ("ecl", &|ecl| {
        matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
    }),
    ("pid", &|pid| pid.len() == 9 && pid.parse::<u32>().is_ok()),
];

fn is_valid_height(hgt: &str) -> bool {
    if let Some(cm) = hgt.strip_suffix("cm") {
        match cm.parse::<u32>() {
            Ok(150..=193) => true,
            _ => false,
        }
    } else if let Some(inches) = hgt.strip_suffix("in") {
        match inches.parse::<u32>() {
            Ok(59..=76) => true,
            _ => false,
        }
    } else {
        false
    }
}

fn is_valid_color(s: &str) -> bool {
    match s.strip_prefix('#') {
        Some(code) if code.len() == 6 && code.chars().all(|c| c.is_ascii_hexdigit()) => true,
        _ => false,
    }
}

impl std::str::FromStr for PassportData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = HashMap::new();

        for pair in s.split_whitespace() {
            let parts: Vec<&str> = pair.split(':').collect();
            match parts.as_slice() {
                [key, value] => {
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
