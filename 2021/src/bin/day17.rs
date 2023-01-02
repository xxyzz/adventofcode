use std::fs;
use regex::Regex;

const DAY: u32 = 17;


fn parse_input(text: &str) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.captures_iter(text).map(|x| i32::from_str_radix(x.get(0).unwrap().as_str(), 10).unwrap()).collect()
}


fn part_one(text: &str) -> i32 {
    let ranges = parse_input(text);
    let min_y = ranges[2];
    // https://reddit.com/r/adventofcode/comments/ri9kdq/2021_day_17_solutions/hovspgy
    min_y.abs() * (min_y.abs() - 1) / 2
}


// fn part_two(text: &str) -> i64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
	let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 45);
    }

    // #[test]
    // fn test_part_two() {
    //     let lines =
    //         fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
    //     assert_eq!(part_two(&lines), );
    // }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
