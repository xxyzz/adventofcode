use std::collections::HashMap;
use std::fs;

const DAY: u32 = 10;

fn part_one(text: &str) -> u64 {
    let mut pre_jolt = 0;
    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 1;
    let mut jolts: Vec<u64> = text
        .lines()
        .map(|line| u64::from_str_radix(line, 10).unwrap())
        .collect();
    jolts.sort();
    for jolt in jolts {
        let diff = jolt - pre_jolt;
        if diff == 1 {
            one_jolt_diff += 1;
        } else if diff == 3 {
            three_jolt_diff += 1;
        }
        pre_jolt = jolt;
    }
    one_jolt_diff * three_jolt_diff
}

// https://github.com/norvig/pytudes/blob/main/ipynb/Advent-2020.ipynb
fn get_arrangements<'a>(
    jolts: &'a [u64],
    pre_jolt: u64,
    cache: &mut HashMap<(&'a [u64], u64), u64>,
) -> u64 {
    let key = (jolts, pre_jolt);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let first = jolts[0];
    let rest = &jolts[1..];
    if first - pre_jolt > 3 {
        cache.insert(key, 0);
        0
    } else if rest.len() == 0 {
        cache.insert(key, 1);
        1
    } else {
        let arragments =
            get_arrangements(rest, first, cache) + get_arrangements(rest, pre_jolt, cache);
        cache.insert(key, arragments);
        arragments
    }
}

fn part_two(text: &str) -> u64 {
    let mut jolts: Vec<u64> = text
        .lines()
        .map(|line| u64::from_str_radix(line, 10).unwrap())
        .collect();
    jolts.sort();
    let mut cache: HashMap<(&[u64], u64), u64> = HashMap::new();
    get_arrangements(&jolts[..], 0, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input1")).expect("Can't read file");
        assert_eq!(part_one(&lines), 35);
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input2")).expect("Can't read file");
        assert_eq!(part_one(&lines), 220);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input1")).expect("Can't read file");
        assert_eq!(part_two(&lines), 8);
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input2")).expect("Can't read file");
        assert_eq!(part_two(&lines), 19208);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
