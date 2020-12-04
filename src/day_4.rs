use regex::Regex;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

static HAIR_COLOR_PATTERN: &str = r"#[0-9|a-f]{6}";
static EYE_COLOR_PATTERN: &str = r"amb|blu|brn|gry|grn|hzl|oth";
static ID_PATTERN: &str = r"^\d{9}$";

pub fn solve(input: &str) {
    let lines = crate::get_lines(input);
    let r_1 = get_passports(&lines).len();
    let r_2 = get_valid_passports(&lines).len();
    assert!(r_1 == 237 && r_2 == 172);
    println!("Day 4. Answer A: {}. Answer B: {}", r_1, r_2);
}

fn get_valid_passports(lines: &Vec<String>) -> Vec<Passport> {
    get_passports(lines)
        .into_iter()
        .filter(|p| has_valid_data(p))
        .collect()
}

fn get_passports(lines: &Vec<String>) -> Vec<Passport> {
    let re = Regex::new(r"(\w{3}:#?[\w|\d]*)").unwrap();
    let mut passport = HashMap::new();
    let mut passports: Vec<HashMap<String, String>> = Vec::default();
    for line in lines {
        if !line.is_empty() {
            for caps in re.captures_iter(line) {
                let v: Vec<&str> = caps[1].split(':').collect();
                passport.insert(v[0].to_string(), v[1].to_string());
            }
        } else {
            passports.push(passport.clone());
            passport.clear();
        }
    }
    passports.push(passport);
    passports
        .into_iter()
        .filter(|passport| has_all_keys(passport))
        .collect()
}

fn has_all_keys(passport: &Passport) -> bool {
    (passport.contains_key("cid") && passport.len() == 8)
        || (!passport.contains_key("cid") && passport.len() == 7)
}

fn has_valid_data(passport: &Passport) -> bool {
    has_valid_year(passport, ("byr", 1920, 2002))
        && has_valid_year(passport, ("iyr", 2010, 2020))
        && has_valid_year(passport, ("eyr", 2020, 2030))
        && has_valid_height(passport)
        && has_valid_hair_color(passport)
        && has_valid_eye_color(passport)
        && has_valid_id(passport)
}

fn has_valid_year(passport: &Passport, policy: (&str, usize, usize)) -> bool {
    passport
        .get(policy.0)
        .and_then(|d| d.parse::<usize>().ok())
        .filter(|&year| year >= policy.1 && year <= policy.2)
        .is_some()
}

fn has_valid_height(passport: &Passport) -> bool {
    let re = Regex::new(r"(\d*)(cm|in)").unwrap();
    passport
        .get("hgt")
        .and_then(|d| re.captures(d))
        .map(|c| (c[1].parse::<usize>().unwrap(), c[2].to_string()))
        .filter(|(value, unit)| {
            (unit == "cm" && (value >= &150 && value <= &193))
                || (unit == "in" && (value >= &59 && value <= &76))
        })
        .is_some()
}

fn has_valid_hair_color(passport: &Passport) -> bool {
    has_valid_value(passport, "hcl", HAIR_COLOR_PATTERN)
}

fn has_valid_eye_color(passport: &Passport) -> bool {
    has_valid_value(passport, "ecl", EYE_COLOR_PATTERN)
}

fn has_valid_id(passport: &Passport) -> bool {
    has_valid_value(passport, "pid", ID_PATTERN)
}

fn has_valid_value(passport: &Passport, key: &str, pattern: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    passport
        .get(key)
        .filter(|value| re.is_match(value))
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passports() {
        let input = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(get_passports(&input).len(), 2);
        assert_eq!(get_valid_passports(&input).len(), 2);
    }

    #[test]
    fn invalid_passports() {
        let input = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(get_valid_passports(&input).len(), 0);
    }

    #[test]
    fn valid_passports() {
        let input = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(get_valid_passports(&input).len(), 4);
    }
}
