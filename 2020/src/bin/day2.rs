use std::fs;

fn part_one(text: &str) -> u32 {
    let mut valid_count = 0;
    for line in text.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = parts[0].split("-").collect();
        let left_range = u32::from_str_radix(range[0], 10).unwrap();
        let right_range = u32::from_str_radix(range[1], 10).unwrap();
        let letter: char = parts[1].chars().collect::<Vec<char>>()[0];
        let passwd = parts[2];
        let mut counter = 0;
        for pass_letter in passwd.chars() {
            if pass_letter == letter {
                counter += 1;
            }
        }
        if left_range <= counter && counter <= right_range {
            valid_count += 1;
        }
    }

    valid_count
}

fn part_two(text: &str) -> u64 {
    let mut valid_count = 0;
    for line in text.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = parts[0].split("-").collect();
        let first_location = usize::from_str_radix(range[0], 10).unwrap() - 1;
        let second_location = usize::from_str_radix(range[1], 10).unwrap() - 1;
        let letter: char = parts[1].chars().collect::<Vec<char>>()[0];
        let passwd = parts[2];
        let chars: Vec<char> = passwd.chars().collect();
        if chars[first_location] == letter && chars[second_location] != letter {
            valid_count += 1;
        } else if chars[second_location] == letter && chars[first_location] != letter {
            valid_count += 1;
        }
    }

    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string("input/day2_test_input").expect("Can't read file");
        assert_eq!(part_one(&lines), 2);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string("input/day2_test_input").expect("Can't read file");
        assert_eq!(part_two(&lines), 1);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day2_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
