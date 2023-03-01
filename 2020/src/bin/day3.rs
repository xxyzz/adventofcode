use std::collections::HashSet;
use std::fs;
use std::iter::zip;

fn parse_map(text: &str) -> (HashSet<(usize, usize)>, usize, usize) {
    let mut tree_map: HashSet<(usize, usize)> = HashSet::new();
    let mut row_num = 0;
    let mut col_num = 0;
    for (row, line) in text.lines().enumerate() {
        col_num = 0;
        for (col, character) in line.chars().enumerate() {
            if character == '#' {
                tree_map.insert((row, col));
            }
            col_num += 1;
        }
        row_num += 1;
    }

    (tree_map, col_num, row_num)
}

fn count_trees(
    tree_map: &HashSet<(usize, usize)>,
    col_num: usize,
    row_num: usize,
    slope_col: usize,
    slope_row: usize,
) -> u32 {
    let mut encounter_trees = 0;
    let mut current_row = slope_row;
    let mut current_col = slope_col;
    while current_row < row_num {
        if tree_map.contains(&(current_row, current_col)) {
            encounter_trees += 1;
        }
        current_row += slope_row;
        current_col += slope_col;
        if current_col >= col_num {
            current_col %= col_num;
        }
    }
    encounter_trees
}

fn part_one(text: &str) -> u32 {
    let (tree_map, col_num, row_num) = parse_map(text);
    count_trees(&tree_map, col_num, row_num, 3, 1)
}

fn part_two(text: &str) -> u32 {
    let mut tree_nums: Vec<u32> = Vec::new();
    let slope_cols = [1, 3, 5, 7, 1];
    let slope_rows = [1, 1, 1, 1, 2];
    let (tree_map, col_num, row_num) = parse_map(text);

    for (slope_col, slope_row) in zip(slope_cols, slope_rows) {
        tree_nums.push(count_trees(
            &tree_map, col_num, row_num, slope_col, slope_row,
        ));
    }
    tree_nums.into_iter().reduce(|acc, e| acc * e).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string("input/day3_test_input").expect("Can't read file");
        assert_eq!(part_one(&lines), 7);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string("input/day3_test_input").expect("Can't read file");
        assert_eq!(part_two(&lines), 336);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day3_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
