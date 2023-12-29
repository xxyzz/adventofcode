use std::{collections::HashMap, fs};

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_beam(
    point: (usize, usize),
    direction: Direction,
    mirror_size: usize,
    mirrors: &HashMap<(usize, usize), char>,
    energized_tails: &mut HashMap<(usize, usize), Vec<Direction>>,
) {
    if energized_tails.contains_key(&point)
        && energized_tails.get(&point).unwrap().contains(&direction)
    {
        return;
    }
    energized_tails
        .entry(point)
        .and_modify(|d| d.push(direction.clone()))
        .or_insert(vec![direction.clone()]);
    if mirrors.contains_key(&point) {
        for (new_point, new_dir) in
            reflect(point, direction, mirror_size, *mirrors.get(&point).unwrap())
        {
            move_beam(new_point, new_dir, mirror_size, mirrors, energized_tails);
        }
    } else if let Some(new_point) = move_beam_in_direction(point, direction.clone(), mirror_size) {
        move_beam(new_point, direction, mirror_size, mirrors, energized_tails);
    }
}

fn reflect(
    point: (usize, usize),
    direction: Direction,
    mirror_size: usize,
    mirror: char,
) -> Vec<((usize, usize), Direction)> {
    let mut reflected_points: Vec<((usize, usize), Direction)> = vec![];
    match mirror {
        '-' => match direction {
            Direction::Up | Direction::Down => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Left, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Left));
                }
                if let Some(new_point) =
                    move_beam_in_direction(point, Direction::Right, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Right));
                }
            }
            _ => {
                if let Some(new_point) =
                    move_beam_in_direction(point, direction.clone(), mirror_size)
                {
                    reflected_points.push((new_point, direction));
                }
            }
        },
        '|' => match direction {
            Direction::Left | Direction::Right => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Up, mirror_size) {
                    reflected_points.push((new_point, Direction::Up));
                }
                if let Some(new_point) = move_beam_in_direction(point, Direction::Down, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Down));
                }
            }
            _ => {
                if let Some(new_point) =
                    move_beam_in_direction(point, direction.clone(), mirror_size)
                {
                    reflected_points.push((new_point, direction));
                }
            }
        },
        '/' => match direction {
            Direction::Up => {
                if let Some(new_point) =
                    move_beam_in_direction(point, Direction::Right, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Right));
                }
            }
            Direction::Right => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Up, mirror_size) {
                    reflected_points.push((new_point, Direction::Up));
                }
            }
            Direction::Down => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Left, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Left));
                }
            }
            Direction::Left => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Down, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Down));
                }
            }
        },
        _ => match direction {
            Direction::Up => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Left, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Left));
                }
            }
            Direction::Right => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Down, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Down));
                }
            }
            Direction::Down => {
                if let Some(new_point) =
                    move_beam_in_direction(point, Direction::Right, mirror_size)
                {
                    reflected_points.push((new_point, Direction::Right));
                }
            }
            Direction::Left => {
                if let Some(new_point) = move_beam_in_direction(point, Direction::Up, mirror_size) {
                    reflected_points.push((new_point, Direction::Up));
                }
            }
        },
    }
    reflected_points
}

fn move_beam_in_direction(
    point: (usize, usize),
    direction: Direction,
    mirror_size: usize,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if point.0 > 0 {
                return Some((point.0 - 1, point.1));
            }
        }
        Direction::Right => {
            if point.1 + 1 < mirror_size {
                return Some((point.0, point.1 + 1));
            }
        }
        Direction::Down => {
            if point.0 + 1 < mirror_size {
                return Some((point.0 + 1, point.1));
            }
        }
        Direction::Left => {
            if point.1 > 0 {
                return Some((point.0, point.1 - 1));
            }
        }
    };
    None
}

fn part_one(text: &str) -> usize {
    let mut mirrors: HashMap<(usize, usize), char> = HashMap::new();
    let mut energized_tails: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut mirror_size = 0;
    text.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c != '.' {
                mirrors.insert((row, col), c);
            }
        });
        mirror_size += 1;
    });
    move_beam(
        (0, 0),
        Direction::Right,
        mirror_size,
        &mirrors,
        &mut energized_tails,
    );
    energized_tails.len()
}

fn part_two(text: &str) -> usize {
    let mut mirrors: HashMap<(usize, usize), char> = HashMap::new();
    let mut energized_tails: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    let mut mirror_size = 0;
    text.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c != '.' {
                mirrors.insert((row, col), c);
            }
        });
        mirror_size += 1;
    });
    let mut max_tails = 0;
    for (row, direction) in [(0, Direction::Down), (mirror_size - 1, Direction::Up)] {
        for col in 0..mirror_size {
            energized_tails.clear();
            move_beam(
                (row, col),
                direction.clone(),
                mirror_size,
                &mirrors,
                &mut energized_tails,
            );
            if energized_tails.len() > max_tails {
                max_tails = energized_tails.len();
            }
        }
    }
    for (col, direction) in [(0, Direction::Right), (mirror_size - 1, Direction::Left)] {
        for row in 0..mirror_size {
            energized_tails.clear();
            move_beam(
                (row, col),
                direction.clone(),
                mirror_size,
                &mirrors,
                &mut energized_tails,
            );
            if energized_tails.len() > max_tails {
                max_tails = energized_tails.len();
            }
        }
    }
    max_tails
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 51);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day16").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
