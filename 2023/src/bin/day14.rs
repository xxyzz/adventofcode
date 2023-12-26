use std::{
    cmp::Ordering::Equal,
    collections::{HashMap, HashSet},
    fs,
};

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

fn part_two(text: &str) -> usize {
    let mut round_rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut cube_rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut mirror_size = 0;
    for (row_index, row) in text.lines().enumerate() {
        for (col_index, rock_type) in row.chars().enumerate() {
            match rock_type {
                'O' => {
                    round_rocks.insert((row_index, col_index));
                }
                '#' => {
                    cube_rocks.insert((row_index, col_index));
                }
                _ => {}
            }
        }
        mirror_size = row_index + 1;
    }

    let mut rotated_cube_rocks = vec![cube_rocks.clone()];
    for _ in 0..3 {
        rotated_cube_rocks.push(
            rotated_cube_rocks
                .last()
                .unwrap()
                .iter()
                .map(|p| rotate_point(p, mirror_size))
                .collect(),
        );
    }

    let mut records: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    for rounds in 1..=1_000_000_000 {
        // move north
        for current_cube_rocks in &rotated_cube_rocks {
            let mut sorted_rocks: Vec<(usize, usize)> = round_rocks.clone().into_iter().collect();
            sorted_rocks.sort_by(|a, b| match a.0.cmp(&b.0) {
                Equal => a.1.cmp(&b.1),
                other => other,
            });
            for point in sorted_rocks {
                let mut current_point = point;
                for check_row in (0..point.0).rev() {
                    let check_point = (check_row, point.1);
                    if round_rocks.contains(&check_point)
                        || current_cube_rocks.contains(&check_point)
                    {
                        break;
                    }
                    current_point = check_point;
                }
                if current_point != point {
                    round_rocks.insert(current_point);
                    round_rocks.remove(&point);
                }
            }
            round_rocks = round_rocks
                .iter()
                .map(|p| rotate_point(p, mirror_size))
                .collect();
        }
        let mut rocks_vec: Vec<(usize, usize)> = round_rocks.clone().into_iter().collect();
        rocks_vec.sort();
        if records.contains_key(&rocks_vec) {
            let loop_first_index = records.get(&rocks_vec).unwrap();
            let target =
                (1_000_000_000 - loop_first_index) % (rounds - loop_first_index) + loop_first_index;
            for (record, index) in &records {
                if *index == target {
                    return record.iter().map(|p| mirror_size - p.0).sum();
                }
            }
            return 0;
        } else {
            records.entry(rocks_vec).or_insert(rounds);
        }
    }
    0
}

fn rotate_point(point: &(usize, usize), mirror_size: usize) -> (usize, usize) {
    // rotate point 90 degrees clockwise
    let row = point.0;
    let col = point.1;
    (col, mirror_size - row - 1)
}

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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 64);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day14").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
