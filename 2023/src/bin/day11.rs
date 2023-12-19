use std::{fs, collections::HashSet};
use std::cmp::{max, min};

struct Point {
    row: i64,
    col: i64,
}

impl Point {
    fn normal_distance(&self, p: &Point) -> i64 {
        (self.row - p.row).abs() + (self.col - p.col).abs()
    }

    fn distance(&self, p: &Point, empty_rows: &HashSet<i64>, empty_cols: &HashSet<i64>) -> i64 {
        let mut distance = self.normal_distance(p);
        for empty_row in empty_rows {
            if *empty_row > min(self.row, p.row) && *empty_row < max(self.row, p.row) {
                distance += 1;
            }
        }
        for empty_col in empty_cols {
            if *empty_col > min(self.col, p.col) && *empty_col < max(self.col, p.col) {
                distance += 1;
            }
        }
        distance
    }
}


fn part_one(text: &str) -> i64 {
    let map: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let mut empty_rows: HashSet<i64> = HashSet::new();
    let mut empty_cols: HashSet<i64> = HashSet::new();
    let mut galaxies: Vec<Point> = vec![];
    for (row_num, row) in map.iter().enumerate() {
        if row.iter().all(|e| *e == '.') {
            empty_rows.insert(row_num as i64);
        }
        for (col_num, point_value) in row.iter().enumerate() {
            if *point_value == '#' {
                galaxies.push(Point{row: row_num as i64, col: col_num as i64});
            }
        }
    }
    for col in 0..map[0].len() {
        if (0..map.len()).all(|row| map[row][col] == '.') {
            empty_cols.insert(col as i64);
        }
    }

    let mut results: i64 = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            results += galaxies[i].distance(&galaxies[j], &empty_rows, &empty_cols);
        }
    }
    results
}

// fn part_two(text: &str) -> i64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 374);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two);
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day11").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
