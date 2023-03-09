use regex::Regex;
use std::collections::HashMap;
use std::fs;

const DAY: u32 = 14;

fn part_one(text: &str) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let re = Regex::new(r"\[(\d+)\] = (\d+)").unwrap();
    let mut set_zero_mask = 0;
    let mut set_one_mask = 0;

    for line in text.lines() {
        if line.starts_with("mask =") {
            set_zero_mask = 0;
            set_one_mask = 0;
            for c in line.chars() {
                set_zero_mask <<= 1;
                set_one_mask <<= 1;
                match c {
                    '0' => set_zero_mask += 1,
                    '1' => set_one_mask += 1,
                    _ => (),
                }
            }
            set_zero_mask = !set_zero_mask;
        } else {
            let capture = re.captures(line).unwrap();
            let address = usize::from_str_radix(capture.get(1).unwrap().as_str(), 10).unwrap();
            let mut value = usize::from_str_radix(capture.get(2).unwrap().as_str(), 10).unwrap();
            value &= set_zero_mask;
            value |= set_one_mask;
            mem.insert(address, value);
        }
    }

    mem.values().sum()
}

// fn part_two(text: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
        assert_eq!(part_one(&lines), 165);
    }

    // #[test]
    // fn test_part_two() {
    //     let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
    //     assert_eq!(part_two(&lines), );
    // }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}")).expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
