use std::collections::HashSet;
use std::fs;

const DAY: u32 = 8;

fn parse_cmds(text: &str) -> Vec<(&str, i32)> {
    let mut commands: Vec<(&str, i32)> = vec![];
    for line in text.lines() {
        let cmd_parts: Vec<&str> = line.split(" ").collect();
        let cmd = cmd_parts[0];
        let value = i32::from_str_radix(cmd_parts[1], 10).unwrap();
        commands.push((cmd, value));
    }
    commands
}

fn simulate(commands: Vec<(&str, i32)>) -> (bool, i32) {
    let mut accumulator: i32 = 0;
    let mut visited_commands: HashSet<i32> = HashSet::new();

    let mut pc: i32 = 0;
    let mut has_infinite_loop = false;
    loop {
        if visited_commands.contains(&pc) {
            has_infinite_loop = true;
            break;
        }
        visited_commands.insert(pc);
        let (cmd, value) = commands[pc as usize];
        match cmd {
            "acc" => accumulator += value,
            "jmp" => pc += value,
            _ => (),
        }
        if cmd != "jmp" {
            pc += 1;
        }
        if pc == commands.len() as i32 {
            break;  // terminate
        }
    }

    (has_infinite_loop, accumulator)
}

fn part_one(text: &str) -> i32 {
    simulate(parse_cmds(text)).1
}

fn part_two(text: &str) -> i32 {
    let commands = parse_cmds(text);
    for (index, (cmd, value)) in commands.iter().enumerate() {
        let mut modifided_commands = commands.clone();
        match *cmd {
            "jmp" => modifided_commands[index] = ("nop", *value),
            "nop" => modifided_commands[index] = ("jmp", *value),
            _ => (),
        }
        let (has_loop, acc) = simulate(modifided_commands);
        if !has_loop {
            return acc;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 5);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 8);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
