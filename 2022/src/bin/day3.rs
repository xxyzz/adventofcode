#![feature(iter_array_chunks)]
use std::collections::HashSet;
use std::fs;

fn part_one(text: &str) -> u32 {
    let mut priorities = 0;

    for line in text.lines() {
        let line = line.trim();
        let line_len = line.len();
        let half_line_len = line_len / 2;
        let first_compartment: HashSet<char> = HashSet::from_iter(line[0..half_line_len].chars());
        let second_compartment: HashSet<char> =
            HashSet::from_iter(line[half_line_len..line_len].chars());

        let duplicate_item = first_compartment
            .intersection(&second_compartment)
            .next()
            .unwrap();
        if duplicate_item.is_lowercase() {
            priorities += *duplicate_item as u32 - 'a' as u32 + 1;
        } else {
            priorities += *duplicate_item as u32 - 'A' as u32 + 27;
        }
    }

    priorities
}

fn part_two(text: &str) -> u32 {
    let mut priorities = 0;

    // nightly feature
    for [line1, line2, line3] in text.lines().array_chunks() {
        let pack1: HashSet<char> = HashSet::from_iter(line1.trim().chars());
        let pack2: HashSet<char> = HashSet::from_iter(line2.trim().chars());
        let pack3: HashSet<char> = HashSet::from_iter(line3.trim().chars());

        let badge_item = pack1
            .intersection(&pack2)
            .find(|it| pack3.contains(it))
            .unwrap();
        if badge_item.is_lowercase() {
            priorities += *badge_item as u32 - 'a' as u32 + 1;
        } else {
            priorities += *badge_item as u32 - 'A' as u32 + 27;
        }
    }

    priorities
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 70);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day3_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
