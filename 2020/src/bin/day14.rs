use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::str::FromStr;

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

// https://stackoverflow.com/questions/44139493/in-rust-what-is-the-proper-way-to-replicate-pythons-repeat-parameter-in-iter
fn kproduct(seq: String, k: usize) -> Vec<Vec<char>> {
    match k {
        0 => vec![],
        1 => vec![seq.chars().collect()],
        2 => iproduct!(seq.chars(), seq.chars())
            .map(|(a, b)| vec![a, b])
            .collect(),
        _ => iproduct!(kproduct(seq.clone(), k - 1), seq.chars())
            .map(|(mut a, b)| {
                a.push(b);
                a
            })
            .collect(),
    }
}

// https://github.com/norvig/pytudes/blob/main/ipynb/Advent-2020.ipynb
fn part_two(text: &str) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let re = Regex::new(r"\[(\d+)\] = (\d+)").unwrap();
    let mut mask: String = String::new();

    for line in text.lines() {
        if line.starts_with("mask = ") {
            mask = line.chars().skip(7).collect::<String>();
        } else {
            let capture = re.captures(line).unwrap();
            let address = usize::from_str_radix(capture.get(1).unwrap().as_str(), 10).unwrap();
            let value = usize::from_str_radix(capture.get(2).unwrap().as_str(), 10).unwrap();
            let float_bits_num = mask.chars().filter(|&c| c == 'X').count();
            for float_bits in kproduct(String::from_str("01").unwrap(), float_bits_num) {
                let mut final_addr: Vec<char> = vec![];
                let mut float_index = 0;
                for (mask_char, origin_addr_char) in
                    zip(mask.chars(), format!("{:036b}", address).chars())
                {
                    match mask_char {
                        '0' => final_addr.push(origin_addr_char),
                        '1' => final_addr.push('1'),
                        _ => {
                            final_addr.push(float_bits[float_index]);
                            float_index += 1;
                        }
                    }
                }
                mem.insert(
                    usize::from_str_radix(final_addr.iter().collect::<String>().as_str(), 2)
                        .unwrap(),
                    value,
                );
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
        assert_eq!(part_one(&lines), 165);
    }

    #[test]
    fn test_product() {
        assert_eq!(
            kproduct(String::from_str("01").unwrap(), 2),
            vec![['0', '0'], ['0', '1'], ['1', '0'], ['1', '1']]
        );
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
        assert_eq!(part_two(&lines), 208);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
