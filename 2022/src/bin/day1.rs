use std::fs;

fn part_one(text: &str) -> u32 {
    let mut max_calories: u32 = 0;
    let mut current_calories: u32 = 0;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().expect("Not number");
        }
    }

    max_calories
}

fn part_two(text: &str) -> u32 {
    let mut calories: Vec<u32> = Vec::new();
    let mut current_calories: u32 = 0;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            calories.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().expect("Not number");
        }
    }

    calories.sort();
    calories.reverse();
    calories[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n\n";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 45000);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day1_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
