use std::fs;

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

// fn part_two(text: &str) -> u32 {
// }

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

    // #[test]
    // fn test_part_two() {
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day3").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
