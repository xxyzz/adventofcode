use std::collections::HashMap;
use std::fs;

fn part_one(text: &str) -> u32 {
    let mut calibration_value_sum: u32 = 0;

    for line in text.lines() {
        let mut current_calibration_value = 0;
        let line = line.trim();
        for letter in line.chars() {
            if letter.is_ascii_digit() {
                current_calibration_value = 10 * letter.to_digit(10).unwrap();
                break;
            }
        }
        for letter in line.chars().rev() {
            if letter.is_ascii_digit() {
                current_calibration_value += letter.to_digit(10).unwrap();
                break;
            }
        }
        calibration_value_sum += current_calibration_value
    }

    calibration_value_sum
}

fn find_first_number(text: &str, backwords: bool) -> u32 {
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let chars_vec: Vec<_> = text.chars().collect();
    let range = if backwords {
        Box::new((0..chars_vec.len()).rev())
    } else {
        Box::new(0..chars_vec.len()) as Box<dyn Iterator<Item = _>>
    };
    for i in range {
        let first_char = &chars_vec[i];
        if first_char.is_ascii_digit() {
            return first_char.to_digit(10).unwrap();
        }

        for (num_str, num_value) in &numbers {
            let substr: String = chars_vec[i..].iter().collect();
            if substr.starts_with(num_str) {
                return *num_value;
            }
        }
    }
    0
}

fn part_two(text: &str) -> u32 {
    let mut calibration_value_sum: u32 = 0;
    for line in text.lines() {
        calibration_value_sum +=
            10 * find_first_number(line, false) + find_first_number(line, true);
    }
    calibration_value_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_1_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_1_INPUT), 142);
    }

    const TEST_2_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_2_INPUT), 281);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day1_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
