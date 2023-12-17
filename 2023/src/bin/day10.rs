use std::{collections::HashSet, fs};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Point {
    row: usize,
    col: usize,
}

fn connect_to(point: &Point, map: &Vec<Vec<char>>) -> Option<Vec<Point>> {
    if let Some(point_type) = get_point_type(point, map) {
        return match point_type {
            '|' => {
                let mut results = vec![Point {
                    row: point.row + 1,
                    ..*point
                }];
                if point.row > 0 {
                    results.push(Point {
                        row: point.row - 1,
                        ..*point
                    });
                }
                Some(results)
            }
            '-' => {
                let mut results = vec![Point {
                    col: point.col + 1,
                    ..*point
                }];
                if point.col > 0 {
                    results.push(Point {
                        col: point.col - 1,
                        ..*point
                    });
                }
                Some(results)
            }
            'L' => {
                let mut results = vec![Point {
                    col: point.col + 1,
                    ..*point
                }];
                if point.row > 0 {
                    results.push(Point {
                        row: point.row - 1,
                        ..*point
                    });
                }
                Some(results)
            }
            'J' => {
                let mut results = vec![];
                if point.row > 0 {
                    results.push(Point {
                        row: point.row - 1,
                        ..*point
                    });
                }
                if point.col > 0 {
                    results.push(Point {
                        col: point.col - 1,
                        ..*point
                    });
                }
                if results.is_empty() {
                    None
                } else {
                    Some(results)
                }
            }
            '7' => {
                let mut results = vec![Point {
                    row: point.row + 1,
                    ..*point
                }];
                if point.col > 0 {
                    results.push(Point {
                        col: point.col - 1,
                        ..*point
                    });
                }
                Some(results)
            }
            'F' => Some(vec![
                Point {
                    row: point.row + 1,
                    ..*point
                },
                Point {
                    col: point.col + 1,
                    ..*point
                },
            ]),
            _ => None,
        };
    }
    None
}

fn find_neighbor(point: &Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = vec![];
    for row_diff in [-1, 0, 1] {
        for col_diff in [-1, 0, 1] {
            if row_diff == 0 && col_diff == 0 {
                continue;
            }
            neighbors.push(Point {
                row: ((point.row as i64) + row_diff) as usize,
                col: ((point.col as i64) + col_diff) as usize,
            });
        }
    }
    neighbors
}

fn get_point_type(point: &Point, map: &Vec<Vec<char>>) -> Option<char> {
    let map_rows = map.len();
    let map_cols = map[0].len();
    if point.row < map_rows && point.col < map_cols {
        return Some(map[point.row][point.col]);
    }
    None
}

fn part_one(text: &str) -> usize {
    let map: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    for (row_num, row) in map.iter().enumerate() {
        for (col_num, point_type) in row.iter().enumerate() {
            if *point_type == 'S' {
                let start = Point {
                    row: row_num,
                    col: col_num,
                };
                let mut path = vec![start.clone()];
                let mut visited_points: HashSet<Point> = HashSet::from_iter(path.clone());

                for neighbor in find_neighbor(&start) {
                    if let Some(neighbor_connections) = connect_to(&neighbor, &map) {
                        if neighbor_connections.contains(&start) {
                            path.push(neighbor.clone());
                            visited_points.insert(neighbor.clone());
                            let mut current_point = neighbor;
                            loop {
                                let mut find_next_point = false;
                                if let Some(next_points) = connect_to(&current_point, &map) {
                                    for next_point in next_points {
                                        if next_point == start {
                                            if path.len() > 2 {
                                                return (path.len() + 1) / 2;
                                            } else {
                                                continue;
                                            }
                                        }
                                        if !visited_points.contains(&next_point) {
                                            visited_points.insert(next_point.clone());
                                            path.push(next_point.clone());
                                            current_point = next_point;
                                            find_next_point = true;
                                            break;
                                        }
                                    }
                                }
                                if !find_next_point {
                                    path.pop();
                                    current_point = path.last().unwrap().clone();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

// fn part_two(text: &str) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_INPUT_1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT_0), 4);
        assert_eq!(part_one(TEST_INPUT_1), 8);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), 0);
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day10").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
