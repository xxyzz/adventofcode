use std::fs;

fn part_one(text: &str) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in text.lines() {
        let commands: Vec<&str> = line.split_whitespace().collect();
        let command = commands[0];
        let value: u32 = commands[1].parse().unwrap();

        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => (),
        }
    }

    depth * horizontal
}

fn part_two(text: &str) -> u32 {
    let mut depth = 0;
    let mut aim = 0;
    let mut horizontal = 0;
    for line in text.lines() {
        let commands: Vec<&str> = line.split_whitespace().collect();
        let command = commands[0];
        let value: u32 = commands[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => (),
        }
    }

    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string("input/day2_test_input").expect("Can't read file");
        assert_eq!(part_one(&lines), 150);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string("input/day2_test_input").expect("Can't read file");
        assert_eq!(part_two(&lines), 900);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day2_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
