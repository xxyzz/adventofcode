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

// fn part_two(text: &str) -> u32 {

// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 142);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), 45000);
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day1_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
