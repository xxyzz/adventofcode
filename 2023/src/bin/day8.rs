use std::{collections::HashMap, fs};

fn parse_input(text: &str) -> (Vec<char>, HashMap<&str, Vec<&str>>) {
    let mut instructions: Vec<char> = Vec::new();
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut is_instractions = true;
    for line in text.lines() {
        if line.is_empty() {
            is_instractions = false;
            continue;
        }
        if is_instractions {
            instructions = line.chars().collect();
        } else {
            let split_parts: Vec<&str> = line.split(" = ").collect();
            let nodes: Vec<&str> = split_parts[1]
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .collect();
            map.entry(split_parts[0]).or_insert(nodes);
        }
    }
    (instructions, map)
}

fn part_one(text: &str) -> u32 {
    let (instructions, map) = parse_input(text);
    let mut location = "AAA";
    let mut steps = 0;
    for instruction in instructions.iter().cycle() {
        let nodes = map.get(location).unwrap();
        match instruction {
            'L' => location = nodes[0],
            'R' => location = nodes[1],
            _ => (),
        }
        steps += 1;
        if location == "ZZZ" {
            break;
        }
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT_1), 2);
        assert_eq!(part_one(TEST_INPUT_2), 6);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_one(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day8").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
