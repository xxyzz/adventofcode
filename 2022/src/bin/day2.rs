use std::fs;

fn part_one(text: &str) -> u32 {
    let mut scores = 0;

    for line in text.lines() {
        let input_choices: Vec<&str> = line.trim().split_whitespace().collect();
        let mut opponent_choice = input_choices[0];
        let my_choice = input_choices[1];

        match my_choice {
            "X" => scores += 1,
            "Y" => scores += 2,
            "Z" => scores += 3,
            _ => (),
        }

        match opponent_choice {
            "A" => opponent_choice = "X",
            "B" => opponent_choice = "Y",
            "C" => opponent_choice = "Z",
            _ => (),
        }

        let diff = my_choice.chars().next().unwrap() as i32
            - opponent_choice.chars().next().unwrap() as i32;
        match diff {
            0 => scores += 3,      // draw
            1 | -2 => scores += 6, // win
            _ => (),               // lose
        }
    }

    scores
}

fn part_two(text: &str) -> u32 {
    let mut scores = 0;

    for line in text.lines() {
        let input_choices: Vec<&str> = line.trim().split_whitespace().collect();
        let opponent_choice = input_choices[0];
        let my_choice = input_choices[1];

        let mut choice_score = 0;
        match opponent_choice {
            "A" => choice_score = 1,
            "B" => choice_score = 2,
            "C" => choice_score = 3,
            _ => (),
        }

        match my_choice {
            "X" => {
                // lose
                choice_score -= 1;
                if choice_score == 0 {
                    choice_score = 3;
                }
            }
            "Y" => scores += 3, // draw
            "Z" => {
                // win
                choice_score += 1;
                if choice_score == 4 {
                    choice_score = 1;
                }
                scores += 6;
            }
            _ => (),
        }
        scores += choice_score;
    }

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 12);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day2_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
