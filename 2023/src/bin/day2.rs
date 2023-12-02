use std::fs;

fn part_one(text: &str) -> u32 {
    let mut sum = 0;

    for line in text.lines() {
        let first_split: Vec<&str> = line.split(": ").collect();
        let game_id: u32 = first_split[0]
            .chars()
            .skip(5)
            .collect::<String>()
            .parse()
            .unwrap();
        let mut possible_bag = true;

        for cube_sets in first_split[1].split("; ") {
            for cube_str in cube_sets.split(", ") {
                let num_and_color: Vec<&str> = cube_str.split(' ').collect();
                let num: u32 = num_and_color[0].parse().unwrap();
                let color = num_and_color[1];
                if (color == "red" && num > 12)
                    || (color == "green" && num > 13)
                    || (color == "blue" && num > 14)
                {
                    possible_bag = false;
                    break;
                }
            }
        }
        if possible_bag {
            sum += game_id;
        }
    }
    sum
}

fn part_two(text: &str) -> u32 {
    let mut sum = 0;

    for line in text.lines() {
        let first_split: Vec<&str> = line.split(": ").collect();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cube_sets in first_split[1].split("; ") {
            for cube_str in cube_sets.split(", ") {
                let num_and_color: Vec<&str> = cube_str.split(' ').collect();
                let num: u32 = num_and_color[0].parse().unwrap();
                let color = num_and_color[1];
                if color == "red" && num > max_red {
                    max_red = num;
                } else if color == "green" && num > max_green {
                    max_green = num;
                } else if color == "blue" && num > max_blue {
                    max_blue = num;
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 2286);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day2_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
