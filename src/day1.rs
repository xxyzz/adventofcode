use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day1_input").expect("Can't read file");

    match part {
        1 => println!("{}", find_max_calories(&lines)),
        2 => println!("{}", top_three_sum(&lines)),
        _ => (),
    }
}

fn find_max_calories(text: &str) -> u32 {
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

fn top_three_sum(text: &str) -> u32 {
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
    fn part_one() {
        assert_eq!(find_max_calories(TEST_INPUT), 24000);
    }

    #[test]
    fn part_two() {
        assert_eq!(top_three_sum(TEST_INPUT), 45000);
    }
}
