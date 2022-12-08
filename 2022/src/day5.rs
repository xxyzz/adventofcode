use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day5_input").expect("Can't read file");

    match part {
        1 => println!("{}", part_one_stack_top(lines)),
        2 => println!("{}", part_two_stack_top(lines)),
        _ => (),
    }
}

fn crate_mover(text: String, move_at_once: bool) -> String {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut stack_nums = 0;
    let mut parsing_stack = true;

    for line in text.lines() {
        if line.is_empty() {
            continue;
        }

        if stack_nums == 0 {
            stack_nums = (line.len() + 1) / 4;
            stacks = vec![vec![]; stack_nums];
        }
        if line.contains("[") {
            for i in (1..line.len()).step_by(4) {
                if !line[i..i + 1].trim().is_empty() {
                    stacks[(i - 1) / 4].insert(0, line[i..i + 1].chars().next().unwrap());
                }
            }
        } else if parsing_stack {
            parsing_stack = false;
        } else {
            let commands: Vec<&str> = line.split_whitespace().collect();
            let move_count: usize = commands[1].parse().unwrap();
            let source: usize = commands[3].parse().unwrap();
            let dest: usize = commands[5].parse().unwrap();

            if !move_at_once {
                for _ in 0..move_count {
                    let item = stacks[source - 1].pop().unwrap();
                    stacks[dest - 1].push(item);
                }
            } else {
                let mut moved_items: Vec<char> = vec![];
                for _ in 0..move_count {
                    let item = stacks[source - 1].pop().unwrap();
                    moved_items.push(item);
                }
                for item in moved_items.iter().rev() {
                    stacks[dest - 1].push(*item);
                }
            }
        }
    }

    let mut top = String::new();
    for s in stacks.iter() {
        top.push(*s.last().unwrap());
    }
    top
}

fn part_one_stack_top(text: String) -> String {
    crate_mover(text, false)
}

fn part_two_stack_top(text: String) -> String {
    crate_mover(text, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn part_one() {
        assert_eq!(part_one_stack_top(String::from(TEST_INPUT)), "CMZ");
    }

    #[test]
    fn part_two() {
        assert_eq!(part_two_stack_top(String::from(TEST_INPUT)), "MCD");
    }
}
