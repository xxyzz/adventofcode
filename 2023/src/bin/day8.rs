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

fn gcd(a: u64, b: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Greatest_common_divisor
    if b == 0 {
        a
    } else if a > b {
        gcd(b, a % b)
    } else {
        gcd(a, b % a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    // https://en.wikipedia.org/wiki/Least_common_multiple
    (a * b) / gcd(a, b)
}

fn part_two(text: &str) -> u64 {
    let (instructions, map) = parse_input(text);
    let start_locations: Vec<_> = map.keys().filter(|l| l.ends_with('A')).collect();
    let mut steps_vec: Vec<u64> = Vec::new();

    for start_location in start_locations.iter() {
        let mut location = **start_location;
        let mut steps = 0;
        for instruction in instructions.iter().cycle() {
            let nodes = map.get(location).unwrap();
            steps += 1;
            match instruction {
                'L' => location = nodes[0],
                'R' => location = nodes[1],
                _ => (),
            }
            if location.ends_with('Z') {
                steps_vec.push(steps);
                break;
            }
        }
    }

    steps_vec.into_iter().reduce(lcm).unwrap()
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

    const TEST_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(8, 12), 4);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT_3), 6);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day8").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
