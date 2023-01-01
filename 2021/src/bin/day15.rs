use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

const DAY: u64 = 15;

fn parse_map(text: &str) -> Vec<Vec<i64>> {
    let mut risk_map = vec![];
    for line in text.lines() {
        risk_map.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect(),
        );
    }
    risk_map
}

#[derive(Eq)]
struct Item {
    risk: i64,
    loc: (usize, usize),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.risk - other.risk {
            0 => Ordering::Equal,
            diff if diff < 0 => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.risk == other.risk
    }
}

fn lowest_risk(risk_map: &Vec<Vec<i64>>) -> i64 {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();
    pq.push(Item {
        risk: 0,
        loc: (0, 0),
    });
    let map_row = risk_map.len();
    let map_col = risk_map[0].len();
    let end = (map_row - 1, map_col - 1);
    let mut min_risks: HashMap<(usize, usize), i64> = HashMap::new();
    min_risks.insert((0, 0), 0);

    while !pq.is_empty() {
        let item = pq.pop().unwrap();
        if visited.contains(&item.loc) {
            continue;
        }
        visited.insert(item.loc);
        if item.loc == end {
            return item.risk;
        }

        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let new_row = item.loc.0 as i64 + dr;
            let new_col = item.loc.1 as i64 + dc;
            let new_loc = (new_row as usize, new_col as usize);
            if 0 <= new_row
                && new_row < map_row as i64
                && 0 <= new_col
                && new_col < map_col as i64
                && !visited.contains(&new_loc)
            {
                let new_risk = item.risk + risk_map[new_row as usize][new_col as usize];
                if new_risk < *min_risks.get(&new_loc).unwrap_or(&i64::MAX) {
                    min_risks.entry(new_loc).and_modify(|v| *v = new_risk);
                    pq.push(Item {
                        risk: new_risk,
                        loc: new_loc,
                    });
                }
            }
        }
    }
    0
}

fn part_one(text: &str) -> i64 {
    let risk_map = parse_map(text);
    lowest_risk(&risk_map)
}

fn expand_risk_map(risk_map: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let origin_row = risk_map.len();
    let origin_col = risk_map[0].len();
    let mut expanded_map = vec![vec![0; origin_col * 5]; origin_row * 5];
    for row in 0..origin_row * 5 {
        for col in 0..origin_col * 5 {
            if row < origin_row && col < origin_col {
                expanded_map[row][col] = risk_map[row][col];
            // print!("{}", risk_map[row][col]);
            } else {
                let last_row = if row >= origin_row {
                    row - origin_row
                } else {
                    row
                };
                let last_col = if col >= origin_col {
                    col - origin_col
                } else {
                    col
                };
                let mut new_risk = expanded_map[last_row][last_col];
                if row >= origin_row {
                    new_risk += 1;
                }
                if new_risk > 9 {
                    new_risk = 1;
                }
                if col >= origin_col {
                    new_risk += 1;
                }
                if new_risk > 9 {
                    new_risk = 1;
                }
                expanded_map[row][col] = new_risk;
                // print!("{}", new_risk);
            }
        }
        // println!();
    }
    expanded_map
}

fn part_two(text: &str) -> i64 {
    let origin_risk_map = parse_map(text);
    let expanded_risk_map = expand_risk_map(&origin_risk_map);
    lowest_risk(&expanded_risk_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 40);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 315);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
