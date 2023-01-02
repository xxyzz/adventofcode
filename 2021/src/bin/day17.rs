use regex::Regex;
use std::fs;

const DAY: u32 = 17;

fn parse_input(text: &str) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.captures_iter(text)
        .map(|x| i32::from_str_radix(x.get(0).unwrap().as_str(), 10).unwrap())
        .collect()
}

fn part_one(text: &str) -> i32 {
    let ranges = parse_input(text);
    let min_y = ranges[2];
    // https://reddit.com/r/adventofcode/comments/ri9kdq/2021_day_17_solutions/hovspgy
    min_y.abs() * (min_y.abs() - 1) / 2
}

fn simulate(
    x: i32,
    y: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    vx: i32,
    vy: i32,
) -> i32 {
    if x <= max_x && x >= min_x && y <= max_y && y >= min_y {
        return 1;
    }
    if x > max_x || y < min_y {
        return 0;
    }
    let new_vx = if vx > 0 {
        vx - 1
    } else if vx < 0 {
        vx + 1
    } else {
        0
    };
    simulate(x + vx, y + vy, min_x, max_x, min_y, max_y, new_vx, vy - 1)
}

fn part_two(text: &str) -> i32 {
    let ranges = parse_input(text);
    let min_x = ranges[0];
    let max_x = ranges[1];
    let min_y = ranges[2];
    let max_y = ranges[3];

    // https://reddit.com/r/adventofcode/comments/ri9kdq/2021_day_17_solutions/how671p
    let mut count = 0;
    for vx in 1..=max_x {
        for vy in min_y..min_y.abs() {
            count += simulate(0, 0, min_x, max_x, min_y, max_y, vx, vy);
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 45);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 112);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
