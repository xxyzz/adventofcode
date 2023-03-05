use std::collections::{HashSet, VecDeque};
use std::fs;

const DAY: u32 = 9;

fn parse_nums(text: &str) -> Vec<i64> {
    text.lines()
        .map(|e| i64::from_str_radix(e, 10).unwrap())
        .collect()
}

fn has_two_sum(nums: &[i64], sum: i64) -> bool {
    let mut nums_set: HashSet<i64> = HashSet::new();
    for num in nums.iter() {
        if nums_set.contains(&(sum - num)) {
            return true;
        }
        nums_set.insert(*num);
    }
    false
}

fn find_invalid_num(numbers: &Vec<i64>, preamble: usize) -> i64 {
    for index in preamble..=numbers.len() - preamble {
        let num_range = &numbers[index..index + preamble];
        let sum = numbers[index + preamble];
        if !has_two_sum(num_range, sum) {
            return sum;
        }
    }
    0
}

fn part_one(text: &str, preamble: usize) -> i64 {
    let numbers = parse_nums(text);
    find_invalid_num(&numbers, preamble)
}

fn part_two(text: &str, preamble: usize) -> i64 {
    let numbers = parse_nums(text);
    let target = find_invalid_num(&numbers, preamble);
    let mut queue = VecDeque::new();
    let mut sum = 0;
    for num in numbers {
	if sum < target {
	    sum += num;
            queue.push_back(num);
        }
        if sum == target && queue.len() >= 2 {
            return queue.iter().min().unwrap() + queue.iter().max().unwrap();
        }
        while sum > target {
            sum -= queue.pop_front().unwrap();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines, 5), 127);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines, 5), 62);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines, 25));
    println!("{}", part_two(&lines, 25));
}
