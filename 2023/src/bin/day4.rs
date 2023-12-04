use std::{collections::HashSet, fs};

fn part_one(text: &str) -> u32 {
    let mut sum = 0;
    for line in text.lines() {
        let mut winning_nums = HashSet::new();
        let first_split: Vec<&str> = line.split(" | ").collect();
        let split_winning_part: Vec<_> = first_split[0].split(": ").collect();
        for winning_num in split_winning_part[1].split(' ') {
            if !winning_num.trim().is_empty() {
                winning_nums.insert(winning_num.trim());
            }
        }
        let mut winning_count = 0;
        for my_num in first_split[1].split(' ') {
            if winning_nums.contains(my_num.trim()) {
                winning_count += 1;
            }
        }
        if winning_count > 0 {
            sum += 2_u32.pow(winning_count - 1);
        }
    }
    sum
}

fn part_two(text: &str) -> u32 {
    let mut winning_cards = Vec::new();
    for line in text.lines() {
        let mut winning_nums = HashSet::new();
        let first_split: Vec<&str> = line.split(" | ").collect();
        let split_winning_part: Vec<_> = first_split[0].split(": ").collect();
        for winning_num in split_winning_part[1].split(' ') {
            if !winning_num.trim().is_empty() {
                winning_nums.insert(winning_num.trim());
            }
        }
        let mut winning_count = 0;
        for my_num in first_split[1].split(' ') {
            if winning_nums.contains(my_num.trim()) {
                winning_count += 1;
            }
        }
        winning_cards.push(winning_count);
    }
    let mut cards = vec![1; winning_cards.len()];
    for (index, winning_card_num) in winning_cards.iter().enumerate() {
        for plus in 1..winning_card_num + 1 {
            cards[index + plus] += cards[index];
        }
    }
    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 30);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day4").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
