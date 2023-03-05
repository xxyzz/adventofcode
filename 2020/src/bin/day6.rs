use std::collections::HashSet;
use std::fs;

const DAY: u32 = 6;

fn part_one(text: &str) -> u32 {
    let mut question_sum = 0;
    for group in text.split("\n\n") {
        let mut question_set: HashSet<char> = HashSet::new();
        for line in group.lines() {
            for question in line.chars() {
                question_set.insert(question);
            }
        }
        question_sum += question_set.len();
    }
    question_sum as u32
}

fn part_two(text: &str) -> u32 {
    let mut question_sum = 0;
    for group in text.split("\n\n") {
        let mut question_set: HashSet<char> =
            HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
        for line in group.lines() {
            let person_questions: HashSet<char> = HashSet::from_iter(line.chars());
            question_set = question_set
                .intersection(&person_questions)
                .map(|&x| x)
                .collect();
        }
        question_sum += question_set.len();
    }
    question_sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 11);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 6);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
