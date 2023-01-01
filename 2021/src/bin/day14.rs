use std::collections::HashMap;
use std::fs;

const DAY: u32 = 14;

fn parse_input(text: &str) -> (Vec<char>, HashMap<&str, &str>) {
    let mut template = "";
    let mut rules = HashMap::new();
    for line in text.lines() {
        if !line.is_empty() {
            if line.contains("->") {
                let rule_values: Vec<&str> = line.split(" -> ").collect();
                rules.insert(rule_values[0], rule_values[1]);
            } else {
                template = line;
            }
        }
    }
    (template.chars().collect(), rules)
}

fn insert_polymer(template: Vec<char>, rules: &HashMap<&str, &str>) -> Vec<char> {
    let mut new_template = template.clone();
    for index in 0..template.len() - 1 {
        let pair = String::from_iter(template[index..index + 2].iter());
        new_template.insert(2 * index + 1, rules[&pair[..]].chars().next().unwrap());
    }
    new_template
}

fn part_one(text: &str) -> u32 {
    let (mut template, rules) = parse_input(text);
    for _ in 0..10 {
        template = insert_polymer(template, &rules);
    }
    let mut polymer_counter: HashMap<&char, u32> = HashMap::new();
    for polymer in template.iter() {
        polymer_counter
            .entry(polymer)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut polymer_nums: Vec<&u32> = polymer_counter.values().collect();
    polymer_nums.sort();
    *polymer_nums.last().unwrap() - *polymer_nums[0]
}

// fn part_two(text: &str) -> u32 {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 1588);
    }

    // #[test]
    // fn test_part_two() {
    //     let lines = fs::read_to_string("input/day{DAY}_test_input").expect("Can't read file");
    //     assert_eq!(part_two(&lines), );
    // }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
