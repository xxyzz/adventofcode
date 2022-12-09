use std::fs;

#[derive(PartialEq, Debug)]
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

fn part_one(text: &str) -> u32 {
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

fn part_two(text: &str) -> u32 {
    let mut pairs = 0;

    for line in text.lines() {
        let (range1, range2) = parse_range(line);

        if range1.start <= range2.end && range2.start <= range1.end {
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
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day4_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
