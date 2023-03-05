use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

const DAY: u32 = 7;

fn find_bags<'a>(
    bags_map: &HashMap<&str, Vec<&'a str>>,
    valid_bags: &mut HashSet<&'a str>,
    target_bag: &str,
) {
    for bag in bags_map.get(target_bag).unwrap_or(&vec![]) {
        valid_bags.insert(bag);
        find_bags(bags_map, valid_bags, bag);
    }
}

fn part_one(text: &str) -> u32 {
    let re = Regex::new(r"([a-z ]+) bags?").unwrap();
    let mut bags_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in text.lines() {
        let bags: Vec<regex::Captures> = re.captures_iter(line).collect();
        let outer_bag = bags[0].get(1).unwrap().as_str().trim();
        for bag in bags[1..].iter() {
            let inner_bag = bag.get(1).unwrap().as_str().trim();
            bags_map
                .entry(&inner_bag)
                .and_modify(|v| v.push(&outer_bag))
                .or_insert(Vec::from_iter([outer_bag]));
        }
    }
    let mut valid_bags: HashSet<&str> = HashSet::new();
    find_bags(&bags_map, &mut valid_bags, "shiny gold");
    valid_bags.len() as u32
}

fn count_bags(bags_map: &HashMap<&str, Vec<(u32, &str)>>, target_bag: &str) -> u32 {
    let mut total_count = 0;
    for (count, bag) in bags_map.get(target_bag).unwrap_or(&vec![]) {
        total_count += count;
        total_count += count * count_bags(bags_map, bag);
    }
    total_count
}

fn part_two(text: &str) -> u32 {
    let re = Regex::new(r"(\d+ )?([a-z ]+) bags?").unwrap();
    let mut bags_map: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for line in text.lines() {
        let bags: Vec<regex::Captures> = re.captures_iter(line).collect();
        let outer_bag = bags[0].get(2).unwrap().as_str();
        for capture in bags[1..].iter() {
            let count = u32::from_str_radix(capture.get(1).unwrap().as_str().trim(), 10).unwrap();
            let inner_bag = capture.get(2).unwrap().as_str();
            bags_map
                .entry(&outer_bag)
                .and_modify(|v| v.push((count, inner_bag)))
                .or_insert(vec![(count, inner_bag)]);
        }
    }

    count_bags(&bags_map, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 4);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 32);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
