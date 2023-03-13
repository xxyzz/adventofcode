use std::collections::HashMap;

fn part_one(starting_numbers: Vec<usize>, nth_num: usize) -> usize {
    let mut pre_num: usize = *starting_numbers.last().unwrap();
    let mut seen_nums: HashMap<usize, usize> = HashMap::new();
    for index in 0..nth_num {
        if index < starting_numbers.len() {
            seen_nums.insert(starting_numbers[index], index);
        } else {
            let new_num = if !seen_nums.contains_key(&pre_num) {
                0
            } else {
                index - 1 - seen_nums.get(&pre_num).unwrap()
            };
            seen_nums.insert(pre_num, index - 1);
            pre_num = new_num;
        }
    }
    pre_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(vec![0, 3, 6], 2020), 436);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_one(vec![0, 3, 6], 30_000_000), 175_594);
    }
}

fn main() {
    println!("{}", part_one(vec![0, 6, 1, 7, 2, 19, 20], 2020));
    // cargo run --release --bin day15
    println!("{}", part_one(vec![0, 6, 1, 7, 2, 19, 20], 30_000_000));
}
