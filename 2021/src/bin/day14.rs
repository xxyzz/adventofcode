use std::collections::HashMap;
use std::fs;

const DAY: u64 = 14;

// https://reddit.com/r/adventofcode/comments/rfzq6f/2021_day_14_solutions/hohc8vc
fn parse_input(
    text: &str,
) -> (
    HashMap<String, u64>,
    HashMap<&str, char>,
    HashMap<char, u64>,
) {
    let mut template = "";
    let mut rules = HashMap::new();
    for line in text.lines() {
        if !line.is_empty() {
            if line.contains("->") {
                let rule_values: Vec<&str> = line.split(" -> ").collect();
                rules.insert(rule_values[0], rule_values[1].chars().last().unwrap());
            } else {
                template = line;
            }
        }
    }
    let mut pair_counter: HashMap<String, u64> = HashMap::new();
    for index in 0..template.len() - 1 {
        pair_counter
            .entry(template[index..index + 2].to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut char_counter: HashMap<char, u64> = HashMap::new();
    for c in template.chars() {
        char_counter
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    (pair_counter, rules, char_counter)
}

fn insert_polymer(
    pair_counter: &mut HashMap<String, u64>,
    rules: &HashMap<&str, char>,
    char_counter: &mut HashMap<char, u64>,
) {
    let pair_counter_clone = pair_counter.clone();
    for pair in pair_counter_clone.keys() {
        if rules.contains_key(pair.as_str()) {
            let pair_count = pair_counter_clone.get(pair).unwrap();
            pair_counter
                .entry(pair.clone())
                .and_modify(|count| *count -= pair_count);
            let new_pair_a = format!("{}{}", pair.chars().nth(0).unwrap(), rules[pair.as_str()]);
            let new_pair_b = format!("{}{}", rules[pair.as_str()], pair.chars().last().unwrap());
            pair_counter
                .entry(new_pair_a)
                .and_modify(|count| *count += pair_count)
                .or_insert(*pair_count);
            pair_counter
                .entry(new_pair_b)
                .and_modify(|count| *count += pair_count)
                .or_insert(*pair_count);
            char_counter
                .entry(rules[pair.as_str()])
                .and_modify(|count| *count += pair_count)
                .or_insert(*pair_count);
        }
    }
}

fn simulate(text: &str, steps: u64) -> u64 {
    let (mut pair_counter, rules, mut char_counter) = parse_input(text);
    for _ in 0..steps {
        insert_polymer(&mut pair_counter, &rules, &mut char_counter);
    }
    let mut polymer_nums: Vec<&u64> = char_counter.values().collect();
    polymer_nums.sort();
    *polymer_nums.last().unwrap() - *polymer_nums[0]
}

fn part_one(text: &str) -> u64 {
    simulate(text, 10)
}

fn part_two(text: &str) -> u64 {
    simulate(text, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 1588);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 2_188_189_693_529);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
