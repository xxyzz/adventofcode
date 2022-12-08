use std::collections::HashSet;
use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day6_input").expect("Can't read file");

    match part {
        1 => println!("{}", part_one(lines)),
        2 => println!("{}", part_two(lines)),
        _ => (),
    }
}

fn find_marker(text: String, mark_len: usize) -> usize {
    for i in 0..text.len() - mark_len + 1 {
        let s: HashSet<char> = HashSet::from_iter(text[i..i + mark_len].chars());
        if s.len() == mark_len {
            return i + mark_len;
        }
    }
    0
}

fn part_one(text: String) -> usize {
    find_marker(text, 4)
}

fn part_two(text: String) -> usize {
    find_marker(text, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part_one(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part_one(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(
            part_one(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );
        assert_eq!(
            part_one(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            11
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part_two(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part_two(String::from("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(
            part_two(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            29
        );
        assert_eq!(
            part_two(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            26
        );
    }
}
