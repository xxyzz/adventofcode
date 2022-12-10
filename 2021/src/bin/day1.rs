use std::fs;

fn part_one(text: &str) -> u32 {
    let values: Vec<u32> = text.lines().map(|v| v.parse().unwrap()).collect();
    let mut result = 0;
    for i in 1..values.len() {
        if values[i] > values[i - 1] {
            result += 1;
        }
    }
    result
}

fn part_two(text: &str) -> u32 {
    let values: Vec<u32> = text.lines().map(|v| v.parse().unwrap()).collect();
    let mut result = 0;
    for i in 0..values.len() - 3 {
        if values[i..i + 3].iter().sum::<u32>() < values[i + 1..i + 4].iter().sum::<u32>() {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string("input/day1_test_input").expect("Can't read file");
        assert_eq!(part_one(&lines), 7);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string("input/day1_test_input").expect("Can't read file");
        assert_eq!(part_two(&lines), 5);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day1_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
