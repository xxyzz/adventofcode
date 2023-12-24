use std::fs;

fn parse_map(text: &str) -> Vec<Vec<Vec<char>>> {
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
    maps
}

fn search_rows(map: &Vec<Vec<char>>) -> u64 {
    for start_row in 0..map.len() - 1 {
        let result = search_from_row(map, start_row);
        if result > 0 {
            return result;
        }
    }
    0
}

fn search_from_row(map: &Vec<Vec<char>>, start_row: usize) -> u64 {
    let mut find_row = true;
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
        return 100 * (start_row + 1) as u64;
    }
    0
}

fn search_cols(map: &Vec<Vec<char>>) -> u64 {
    for start_col in 0..map[0].len() - 1 {
        let result = search_from_col(map, start_col);
        if result > 0 {
            return result;
        }
    }
    0
}

fn search_from_col(map: &Vec<Vec<char>>, start_col: usize) -> u64 {
    let mut find_col = true;
    let mut left_col = start_col;
    let mut right_col = start_col + 1;
    loop {
        for row in map {
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
        return (start_col + 1) as u64;
    }
    0
}

fn process_map(map: &Vec<Vec<char>>) -> u64 {
    let result = search_rows(map);
    if result > 0 {
        return result;
    }
    search_cols(map)
}

fn part_one(text: &str) -> u64 {
    parse_map(text).iter().map(process_map).sum()
}

fn part_two(text: &str) -> u64 {
    let mut results = 0;
    for map in parse_map(text) {
        for start_row in 0..map.len() - 1 {
            let mut find_row = false;
            let mut first_row = start_row;
            let mut second_row = start_row + 1;
            loop {
                if map[first_row] != map[second_row] {
                    if map[first_row]
                        .iter()
                        .zip(map[second_row].iter())
                        .map(|(a, b)| if a != b { 1 } else { 0 })
                        .sum::<u64>()
                        == 1
                    {
                        let mut new_map = map.clone();
                        new_map[second_row] = map[first_row].clone();
                        let new_result = search_from_row(&new_map, start_row);
                        if new_result > 0 {
                            results += new_result;
                            find_row = true;
                        } else {
                            new_map = map.clone();
                            new_map[first_row] = map[second_row].clone();
                            let new_result = search_from_row(&new_map, start_row);
                            if new_result > 0 {
                                results += new_result;
                                find_row = true;
                            }
                        }
                    }
                    break;
                }
                if first_row == 0 || second_row == map.len() - 1 {
                    break;
                }
                first_row -= 1;
                second_row += 1;
            }
            if find_row {
                break;
            }
        }

        for start_col in 0..map[0].len() - 1 {
            let mut find_col = false;
            let mut left_col = start_col;
            let mut right_col = start_col + 1;
            loop {
                let mut diff_num = 0;
                for row in &map {
                    if row[left_col] != row[right_col] {
                        diff_num += 1;
                        if diff_num > 1 {
                            break;
                        }
                    }
                }
                if diff_num == 1 {
                    let mut new_map = map.clone();
                    for row in &mut new_map {
                        row[left_col] = row[right_col];
                    }
                    let new_result = search_from_col(&new_map, start_col);
                    if new_result > 0 {
                        results += new_result;
                        find_col = true;
                    } else {
                        new_map = map.clone();
                        for row in &mut new_map {
                            row[right_col] = row[left_col];
                        }
                        let new_result = search_from_col(&new_map, start_col);
                        if new_result > 0 {
                            results += new_result;
                            find_col = true;
                        }
                    }
                    break;
                }
                if left_col == 0 || right_col == map[0].len() - 1 {
                    break;
                }
                left_col -= 1;
                right_col += 1;
            }
            if find_col {
                break;
            }
        }
    }
    results
}

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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 400);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day13").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
