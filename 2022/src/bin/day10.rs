use std::fs;

fn part_one(text: &str) -> i64 {
    let mut x_value = 1;
    let mut cycles = 0;
    let mut check_cycle = 20;
    let mut signal_strengths = 0;

    for line in text.lines() {
        if line.starts_with("noop") {
            check_cycle_value(
                &mut cycles,
                &mut check_cycle,
                &mut signal_strengths,
                &x_value,
            );
        } else {
            check_cycle_value(
                &mut cycles,
                &mut check_cycle,
                &mut signal_strengths,
                &x_value,
            );
            check_cycle_value(
                &mut cycles,
                &mut check_cycle,
                &mut signal_strengths,
                &x_value,
            );
            x_value += line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }

    signal_strengths
}

fn check_cycle_value(
    cycles: &mut i64,
    check_cycle: &mut i64,
    signal_strengths: &mut i64,
    x_value: &i64,
) {
    *cycles += 1;
    if cycles == check_cycle {
        *signal_strengths += *cycles * *x_value;
        *check_cycle += 40;
    }
}

fn part_two(text: &str) {
    let mut x_value = 1;
    let mut cycles = 0;

    for line in text.lines() {
        if line.starts_with("noop") {
            print_screen(&mut cycles, &x_value);
        } else {
            print_screen(&mut cycles, &x_value);
            print_screen(&mut cycles, &x_value);
            x_value += line
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
        }
    }
}

fn print_screen(cycles: &mut i64, x_value: &i64) {
    *cycles += 1;
    let print_position = (*cycles - 1) % 40;
    if [x_value - 1, *x_value, x_value + 1].contains(&print_position) {
        print!("#");
    } else {
        print!(" ");
    }
    if print_position == 39 {
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = fs::read_to_string("input/day10_test_input").expect("Can't read file");
        assert_eq!(part_one(&test_input), 13140);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day10_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    part_two(&lines);
}
