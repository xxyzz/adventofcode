use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day4_input").expect("Can't read file");

    match part {
        1 => println!("{}", fully_contain_pairs(lines)),
        2 => println!("{}", overlap_pairs(lines)),
        _ => (),
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

fn parse_range(line: &str) -> (Range, Range) {
    let line = line.trim();
    let mut ranges = line.split(",");
    let mut range1 = ranges.next().unwrap().split("-");
    let mut range2 = ranges.next().unwrap().split("-");
    let range1_start: u32 = range1.next().unwrap().parse().unwrap();
    let range1_end: u32 = range1.next().unwrap().parse().unwrap();
    let range2_start: u32 = range2.next().unwrap().parse().unwrap();
    let range2_end: u32 = range2.next().unwrap().parse().unwrap();

    (
        Range {
            start: range1_start,
            end: range1_end,
        },
        Range {
            start: range2_start,
            end: range2_end,
        },
    )
}

fn fully_contain_pairs(text: String) -> u32 {
    let mut pairs = 0;

    for line in text.lines() {
        let (range1, range2) = parse_range(line);

        if (range1.start <= range2.start && range1.end >= range2.end)
            || (range2.start <= range1.start && range2.end >= range1.end)
        {
            pairs += 1;
        }
    }

    pairs
}

fn overlap_pairs(text: String) -> u32 {
    let mut pairs = 0;

    for line in text.lines() {
        let (range1, range2) = parse_range(line);

        if (range1.start >= range2.start && range1.start <= range2.end)
            || (range1.end >= range2.start && range1.end <= range2.end)
            || (range2.start >= range1.start && range2.start <= range1.end)
            || (range2.end >= range1.start && range2.end <= range1.end)
        {
            pairs += 1;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_parse_range() {
        assert_eq!(
            parse_range("2-4,6-8"),
            (Range { start: 2, end: 4 }, Range { start: 6, end: 8 })
        );
    }

    #[test]
    fn part_one() {
        assert_eq!(fully_contain_pairs(String::from(TEST_INPUT)), 2);
    }

    #[test]
    fn part_two() {
        assert_eq!(overlap_pairs(String::from(TEST_INPUT)), 4);
    }
}
