use regex::Regex;
use std::collections::HashSet;
use std::fs;

const DAY: u32 = 4;

fn part_one(text: &str) -> u32 {
    let check_fields = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let mut valid_num = 0;
    for passport in text.split("\n\n") {
        let mut fields: HashSet<&str> = HashSet::new();
        for line in passport.lines() {
            for field in line.split(" ") {
                let field_name: &str = field.split(":").collect::<Vec<&str>>()[0];
                fields.insert(field_name);
            }
        }
        if check_fields
            .intersection(&fields)
            .collect::<HashSet<&&str>>()
            .len()
            == check_fields.len()
        {
            valid_num += 1;
        }
    }
    valid_num
}

fn check_field_value(name: &str, value: &str) -> bool {
    match name {
        "byr" => {
            let numeric_value = u32::from_str_radix(value, 10).unwrap();
            if 1920 <= numeric_value && numeric_value <= 2002 {
                true
            } else {
                false
            }
        }
        "iyr" => {
            let numeric_value = u32::from_str_radix(value, 10).unwrap();
            if 2010 <= numeric_value && numeric_value <= 2020 {
                true
            } else {
                false
            }
        }
        "eyr" => {
            let numeric_value = u32::from_str_radix(value, 10).unwrap();
            if 2020 <= numeric_value && numeric_value <= 2030 {
                true
            } else {
                false
            }
        }
        "hgt" => {
            if value.ends_with("cm") {
                let height_chars: Vec<char> = value.chars().collect();
                let height_str: String = height_chars[..(height_chars.len() - 2)].iter().collect();
                let height = u32::from_str_radix(&height_str, 10).unwrap();
                if 150 <= height && height <= 193 {
                    true
                } else {
                    false
                }
            } else if value.ends_with("in") {
                let height_chars: Vec<char> = value.chars().collect();
                let height_str: String = height_chars[..(height_chars.len() - 2)].iter().collect();
                let height = u32::from_str_radix(&height_str, 10).unwrap();
                if 59 <= height && height <= 76 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        "hcl" => {
            let re = Regex::new(r"^#[\da-f]{6}$").unwrap();
            re.is_match(value)
        }
        "ecl" => {
            let valid_eye_colors: HashSet<&str> =
                HashSet::from_iter(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
            valid_eye_colors.contains(value)
        }
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(value)
        }
        "cid" => true,
        _ => false,
    }
}

fn part_two(text: &str) -> u32 {
    let check_fields = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let mut valid_num = 0;
    for passport in text.split("\n\n") {
        let mut fields: HashSet<&str> = HashSet::new();
        let mut is_valid = true;
        for line in passport.lines() {
            for field in line.split(" ") {
                let field_parts = field.split(":").collect::<Vec<&str>>();
                let field_name = field_parts[0];
                let field_value = field_parts[1];
                if !check_field_value(field_name, field_value) {
                    is_valid = false;
                    break;
                }
                fields.insert(field_name);
            }
        }
        if is_valid
            && check_fields
                .intersection(&fields)
                .collect::<HashSet<&&str>>()
                .len()
                == check_fields.len()
        {
            valid_num += 1;
        }
    }
    valid_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 2);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 2);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
