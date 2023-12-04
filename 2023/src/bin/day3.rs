use std::{collections::HashMap, fs};

fn is_valid_number(row_num: i64, start_col: i64, length: i64, matrix: &Vec<Vec<char>>) -> bool {
    let matrix_rows = matrix.len() as i64;
    let matrix_cols = matrix[0].len() as i64;

    for check_col in start_col..start_col + length {
        for move_row in [-1, 0, 1] {
            for move_col in [-1, 0, 1] {
                let new_col = check_col + move_col;
                if move_row == 0 && new_col >= start_col && new_col < start_col + length {
                    continue;
                }
                let new_row = row_num + move_row;
                if new_col >= 0
                    && new_col < matrix_cols
                    && new_row >= 0
                    && new_row < matrix_rows
                    && !matrix[new_row as usize][new_col as usize].is_ascii_digit()
                    && matrix[new_row as usize][new_col as usize] != '.'
                {
                    return true;
                }
            }
        }
    }
    false
}

fn part_one(text: &str) -> u32 {
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        matrix.push(line.chars().collect());
    }
    let matrix_cols = matrix[0].len();
    for (row_num, row) in matrix.iter().enumerate() {
        let mut current_num = String::new();
        let mut start_col = 0;
        for (col_num, character) in row.iter().enumerate() {
            if character.is_ascii_digit() {
                if current_num.is_empty() {
                    start_col = col_num;
                }
                current_num.push_str(&character.to_string());
            }

            if !current_num.is_empty()
                && (!character.is_ascii_digit() || col_num == matrix_cols - 1)
            {
                if is_valid_number(
                    row_num as i64,
                    start_col as i64,
                    current_num.len() as i64,
                    &matrix,
                ) {
                    sum += current_num.parse::<u32>().unwrap();
                }

                current_num.clear();
            }
        }
    }
    sum
}

fn find_near_gear(
    row_num: i64,
    start_col: i64,
    length: i64,
    matrix: &Vec<Vec<char>>,
) -> Option<(i64, i64)> {
    let matrix_rows = matrix.len() as i64;
    let matrix_cols = matrix[0].len() as i64;

    for check_col in start_col..start_col + length {
        for move_row in [-1, 0, 1] {
            for move_col in [-1, 0, 1] {
                let new_col = check_col + move_col;
                if move_row == 0 && new_col >= start_col && new_col < start_col + length {
                    continue;
                }
                let new_row = row_num + move_row;
                if new_col >= 0
                    && new_col < matrix_cols
                    && new_row >= 0
                    && new_row < matrix_rows
                    && matrix[new_row as usize][new_col as usize] == '*'
                {
                    return Some((new_row, new_col));
                }
            }
        }
    }
    None
}

fn part_two(text: &str) -> i64 {
    let mut sum = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in text.lines() {
        matrix.push(line.chars().collect());
    }
    let matrix_cols = matrix[0].len();
    let mut gear_numbers: HashMap<(i64, i64), Vec<i64>> = HashMap::new();
    for (row_num, row) in matrix.iter().enumerate() {
        let mut current_num_str = String::new();
        let mut start_col = 0;
        for (col_num, character) in row.iter().enumerate() {
            if character.is_ascii_digit() {
                if current_num_str.is_empty() {
                    start_col = col_num;
                }
                current_num_str.push_str(&character.to_string());
            }

            if !current_num_str.is_empty()
                && (!character.is_ascii_digit() || col_num == matrix_cols - 1)
            {
                let result = find_near_gear(
                    row_num as i64,
                    start_col as i64,
                    current_num_str.len() as i64,
                    &matrix,
                );
                let current_num: i64 = current_num_str.parse().unwrap();
                if let Some(gear) = result {
                    gear_numbers
                        .entry(gear)
                        .and_modify(|e| e.push(current_num))
                        .or_insert(vec![current_num]);
                };

                current_num_str.clear();
            }
        }
    }
    for ratios in gear_numbers.values() {
        if ratios.len() == 2 {
            sum += ratios.iter().product::<i64>();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 467835);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day3").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
