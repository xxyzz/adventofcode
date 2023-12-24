use std::fs;

fn part_one(text: &str) -> usize {
    let mut map: Vec<Vec<char>> = vec![];
    for line in text.lines() {
        for (col, rock) in line.chars().enumerate() {
            if col >= map.len() {
                map.push(vec![]);
            }
            map[col].push(rock);
        }
    }

    let mut result = 0;
    for col in map.iter_mut() {
        for (row_index, rock) in col.clone().iter().enumerate() {
            if *rock != 'O' {
                continue;
            }
            if row_index == 0 {
                result += col.len();
                continue;
            }
            let mut current_row = row_index - 1;
            loop {
                if col[current_row] != '.' || current_row == 0 {
                    if current_row == 0 && col[0] == '.' {
                        col[0] = 'O';
                        col[row_index] = '.';
                        result += col.len();
                    } else if current_row + 1 != row_index {
                        col[current_row + 1] = 'O';
                        col[row_index] = '.';
                        result += col.len() - current_row - 1;
                    } else {
                        result += col.len() - row_index;
                    }
                    break;
                }
                current_row -= 1;
            }
        }
    }
    result
}

// fn print_map(map: &Vec<Vec<char>>) {
//     for row in 0..map[0].len() {
//         for col in map {
//             print!("{}", col[row]);
//         }
//         print!("\n");
//     }
// }

// fn part_two(text: &str) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 136);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day14").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
