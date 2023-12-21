use std::fs;

fn resolve(records: Vec<char>, rest_groups: Vec<u64>, current_length: u64) -> u64 {
    if records.is_empty() {
        if rest_groups.is_empty() && current_length == 0 {
            return 1;
        }
        if rest_groups.len() == 1 && current_length == rest_groups[0] {
            return 1;
        }
        return 0;
    }
    if !rest_groups.is_empty() && current_length > rest_groups[0] {
        return 0;
    }
    if rest_groups.is_empty() && current_length > 0 {
        return 0;
    }

    let record = records[0];
    let mut total = 0;
    if record == '#' || record == '?' {
        total += resolve(
            records[1..].to_vec(),
            rest_groups.clone(),
            current_length + 1,
        );
    }
    if record == '.' || record == '?' {
        if current_length == 0 {
            total += resolve(records[1..].to_vec(), rest_groups, 0);
        } else if !rest_groups.is_empty() && rest_groups[0] == current_length {
            total += resolve(records[1..].to_vec(), rest_groups[1..].to_vec(), 0);
        }
    }
    total
}

fn part_one(text: &str) -> u64 {
    // copied from https://github.com/mebeim/aoc/blob/master/2023/README.md#day-12---hot-springs
    let mut result = 0;
    for line in text.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let groups: Vec<u64> = split[1].split(',').map(|e| e.parse().unwrap()).collect();
        let records = split[0].chars().collect();
        result += resolve(records, groups, 0);
    }
    result
}

// fn part_two(text: &str) -> i64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 21);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day12").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
