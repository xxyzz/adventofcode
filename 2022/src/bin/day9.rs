use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn part_one(text: &str) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut visited_points: HashSet<Point> = HashSet::new();
    visited_points.insert(tail);

    for line in text.lines() {
        let commands: Vec<&str> = line.split_whitespace().collect();
        let direction = commands[0];
        let steps: u64 = commands[1].parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => (),
            }

            drag(&head, &mut tail, &mut visited_points);
        }
    }

    visited_points.len()
}

fn part_two(text: &str) -> usize {
    let mut head = Point { x: 0, y: 0 };
    let mut tails = [Point { x: 0, y: 0 }; 9];
    let mut visited_points: Vec<HashSet<Point>> = vec![];
    for i in 0..9 {
        visited_points.push(HashSet::new());
        visited_points[i].insert(Point { x: 0, y: 0 });
    }

    for line in text.lines() {
        let commands: Vec<&str> = line.split_whitespace().collect();
        let direction = commands[0];
        let steps: u64 = commands[1].parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => (),
            }

            drag(&mut head, &mut tails[0], &mut visited_points[0]);
            for i in 1..9 {
                drag(&tails[i - 1].clone(), &mut tails[i], &mut visited_points[i]);
            }
        }
    }

    visited_points[8].len()
}

fn drag(head: &Point, tail: &mut Point, visited_points: &mut HashSet<Point>) {
    if point_distance(&head, &tail) >= 2.0 {
        if head.y > tail.y {
            tail.y += 1;
        } else if head.y < tail.y {
            tail.y -= 1;
        }
        if head.x < tail.x {
            tail.x -= 1;
        } else if head.x > tail.x {
            tail.x += 1;
        }
        visited_points.insert(*tail);
    }
}

fn point_distance(point_a: &Point, point_b: &Point) -> f64 {
    return (((point_a.x - point_b.x).pow(2) + (point_a.y - point_b.y).pow(2)) as f64).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = fs::read_to_string("input/day9_test_input").expect("Can't read file");
        assert_eq!(part_one(&test_input), 13);
    }

    #[test]
    fn test_part_two() {
        let test_input = fs::read_to_string("input/day9_test_input").expect("Can't read file");
        assert_eq!(part_two(&test_input), 1);
    }

    #[test]
    fn test_part_two_again() {
        let test_input = fs::read_to_string("input/day9_test_two_input").expect("Can't read file");
        assert_eq!(part_two(&test_input), 36);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day9_input").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
