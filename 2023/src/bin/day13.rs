use std::fs;

fn part_one(text: &str) -> u64 {
    let mut maps: Vec<Vec<Vec<char>>> = vec![];
    let mut current_map: Vec<Vec<char>> = vec![];
    for line in text.lines() {
        if line.is_empty() {
            maps.push(current_map.clone());
            current_map.clear();
            continue;
        }
        current_map.push(line.chars().collect());
    }
    maps.push(current_map.clone());
    let mut results = 0_u64;
    for map in maps {
        let mut find_row = true;
        for start_row in 0..map.len() - 1 {
            find_row = true;
            let mut first_row = start_row;
            let mut second_row = start_row + 1;
            loop {
                if map[first_row] != map[second_row] {
                    find_row = false;
                    break;
                }
                if first_row == 0 || second_row == map.len() - 1 {
                    break;
                }
                first_row -= 1;
                second_row += 1;
            }
            if find_row {
                results += 100 * (start_row + 1) as u64;
                break;
            }
        }
        if find_row {
            continue;
        }
        for start_col in 0..map[0].len() - 1 {
            let mut find_col = true;
            let mut left_col = start_col;
            let mut right_col = start_col + 1;
            loop {
                for row in &map {
                    if row[left_col] != row[right_col] {
                        find_col = false;
                        break;
                    }
                }
                if !find_col {
                    break;
                }
                if left_col == 0 || right_col == map[0].len() - 1 {
                    break;
                }
                left_col -= 1;
                right_col += 1;
            }
            if find_col {
                results += (start_col + 1) as u64;
                break;
            }
        }
    }
    results
}

// fn part_two(text: &str) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 405);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day13").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
