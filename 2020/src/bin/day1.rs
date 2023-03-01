use std::collections::HashSet;
use std::fs;

fn part_one(text: &str) -> u32 {
    let values: HashSet<u32> = text.lines().map(|v| v.parse().unwrap()).collect();
    for value in &values {
        let other_value = 2020 - value;
        if values.contains(&other_value) {
            return value * other_value;
        }
    }
    0
}

fn part_two(text: &str) -> i64 {
    let values: Vec<i64> = text.lines().map(|v| v.parse().unwrap()).collect();
    for index1 in 0..values.len() - 1 {
        let rest_sum = 2020 - values[index1];
        let mut num_set: HashSet<i64> = HashSet::new();
        for index2 in index1 + 1..values.len() {
            if num_set.contains(&(rest_sum - values[index2])) {
                return values[index1] * values[index2] * (rest_sum - values[index2]);
            } else {
                num_set.insert(values[index2]);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string("input/day1_test_input").expect("Can't read file");
        assert_eq!(part_one(&lines), 514579);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string("input/day1_test_input").expect("Can't read file");
        assert_eq!(part_two(&lines), 241861950);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day1_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
