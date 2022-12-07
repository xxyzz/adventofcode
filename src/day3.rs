use std::collections::HashSet;
use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day3_input").expect("Can't read file");

    match part {
        1 => println!("{}", part_one_priorities(lines)),
        2 => println!("{}", part_two_priorities(lines)),
        _ => (),
    }
}

fn part_one_priorities(text: String) -> u32 {
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

fn part_two_priorities(text: String) -> u32 {
    let mut priorities = 0;

    for [line1, line2, line3] in text.lines().array_chunks() {  // nightly feature
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
    fn part_one() {
        assert_eq!(part_one_priorities(String::from(TEST_INPUT)), 157);
    }

    #[test]
    fn part_two() {
        assert_eq!(part_two_priorities(String::from(TEST_INPUT)), 70);
    }
}
