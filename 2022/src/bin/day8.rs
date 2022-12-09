use std::collections::HashSet;
use std::fs;

fn part_one(text: &str) -> usize {
    let trees = scan_trees(text);
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let row_num = trees.len();
    let column_num = trees[0].len();

    // Edge trees
    for row in 0..row_num {
        for column in [0, column_num - 1].iter() {
            visible_trees.insert((row, *column));
        }
    }
    for column in 0..column_num {
        for row in [0, row_num - 1].iter() {
            visible_trees.insert((*row, column));
        }
    }

    for row in 1..row_num - 1 {
        // Scan rows from left to right
        let mut left_tree = trees[row][0];
        for column in 1..column_num - 1 {
            let current_tree = trees[row][column];
            if current_tree > left_tree {
                visible_trees.insert((row, column));
                left_tree = current_tree;
            }
        }
        // Scan rows from right to left
        let mut right_tree = trees[row][column_num - 1];
        for column in (1..column_num - 1).rev() {
            let current_tree = trees[row][column];
            if current_tree > right_tree {
                visible_trees.insert((row, column));
                right_tree = current_tree;
            }
        }
    }

    for column in 1..column_num - 1 {
        // Scan columns from top to bottom
        let mut top_tree = trees[0][column];
        for row in 1..row_num - 1 {
            let current_tree = trees[row][column];
            if current_tree > top_tree {
                visible_trees.insert((row, column));
                top_tree = current_tree;
            }
        }
        // Scan columns from bttom to top
        let mut bottom_tree = trees[row_num - 1][column];
        for row in (1..row_num - 1).rev() {
            let current_tree = trees[row][column];
            if current_tree > bottom_tree {
                visible_trees.insert((row, column));
                bottom_tree = current_tree;
            }
        }
    }

    // Print visible trees
    // for row in 0..row_num {
    //     for col in 0..column_num {
    //         if visible_trees.contains(&(row, col)) {
    //             print!(".");
    //         } else {
    //             print!(" ");
    //         }
    //     }
    //     print!("\n");
    // }
    visible_trees.len()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part_two(text: &str) -> usize {
    let trees = scan_trees(text);
    let row_num = trees.len();
    let column_num = trees[0].len();
    let mut max_scenic_score = 0;

    for row in 1..row_num - 1 {
        for col in 1..column_num - 1 {
            let scenic_score = visable_trees(&trees, row, col, Direction::Up)
                * visable_trees(&trees, row, col, Direction::Down)
                * visable_trees(&trees, row, col, Direction::Right)
                * visable_trees(&trees, row, col, Direction::Left);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    max_scenic_score
}

fn visable_trees(trees: &Vec<Vec<usize>>, row: usize, col: usize, direction: Direction) -> usize {
    let row_num = trees.len();
    let col_num = trees[0].len();
    let mut visable_tree_num = 0;
    let mut row_index = row;
    let mut col_index = col;
    match direction {
        Direction::Up => row_index = row - 1,
        Direction::Down => row_index = row + 1,
        Direction::Left => col_index = col - 1,
        Direction::Right => col_index = col + 1,
    }

    while col_index < col_num && row_index < row_num {
        visable_tree_num += 1;
        if trees[row_index][col_index] >= trees[row][col] {
            break;
        }
        match direction {
            Direction::Up => {
                if row_index == 0 {
                    break;
                } else {
                    row_index -= 1;
                }
            }
            Direction::Down => row_index += 1,
            Direction::Left => {
                if col_index == 0 {
                    break;
                } else {
                    col_index -= 1;
                }
            }
            Direction::Right => col_index += 1,
        }
    }

    visable_tree_num
}

fn scan_trees(text: &str) -> Vec<Vec<usize>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|v| v.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = fs::read_to_string("input/day8_test_input").expect("Can't read file");
        assert_eq!(part_one(&test_input), 21);
    }

    #[test]
    fn test_part_two() {
        let test_input = fs::read_to_string("input/day8_test_input").expect("Can't read file");
        assert_eq!(part_two(&test_input), 8);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day8_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
