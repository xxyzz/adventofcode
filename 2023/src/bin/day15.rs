use std::{fs, str::Chars};

fn hash(chars: Chars) -> u64 {
    let mut current_value = 0;
    for c in chars {
        current_value += c as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn part_one(text: &str) -> u64 {
    if let Some(line) = text.lines().next() {
        return line.split(',').map(|step| hash(step.chars())).sum();
    }
    0
}

// fn part_two(text: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 1320);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day15").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
