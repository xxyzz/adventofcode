use std::collections::HashSet;
use std::fs;

fn find_marker(text: &str, mark_len: usize) -> usize {
    for i in 0..text.len() - mark_len + 1 {
        let s: HashSet<char> = HashSet::from_iter(text[i..i + mark_len].chars());
        if s.len() == mark_len {
            return i + mark_len;
        }
    }
    0
}

fn part_one(text: &str) -> usize {
    find_marker(text, 4)
}

fn part_two(text: &str) -> usize {
    find_marker(text, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day6_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
