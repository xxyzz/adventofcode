use std::fs;

const DAY: u32 = 12;

// https://github.com/norvig/pytudes/blob/main/ipynb/Advent-2020.ipynb
fn rotate(degrees: isize, x: isize, y: isize) -> (isize, isize) {
    if degrees % 360 == 0 {
        (x, y)
    } else {
        rotate(degrees - 90, y, -x)
    }
}

fn move_forward(distance: isize, x: isize, y: isize, dx: isize, dy: isize) -> (isize, isize) {
    // move distance in the (dx, dy) direction from (x, y)
    (x + distance * dx, y + distance * dy)
}

fn part_one(text: &str) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut direction_x: isize = 1;
    let mut direction_y: isize = 0;
    for line in text.lines() {
        let chars: Vec<char> = line.chars().collect();
        let value =
            isize::from_str_radix(chars[1..].iter().collect::<String>().as_str(), 10).unwrap();
        match chars[0] {
            'N' => y += value,
            'S' => y -= value,
            'E' => x += value,
            'W' => x -= value,
            'R' => (direction_x, direction_y) = rotate(value, direction_x, direction_y),
            'L' => (direction_x, direction_y) = rotate(-value, direction_x, direction_y),
            'F' => {
                (x, y) = move_forward(value, x, y, direction_x, direction_y);
            }
            _ => (),
        }
    }

    x.abs() + y.abs()
}

fn part_two(text: &str) -> isize {
    let mut ship_x: isize = 0;
    let mut ship_y: isize = 0;
    let mut waypoint_x: isize = 10;
    let mut waypoint_y: isize = 1;
    for line in text.lines() {
        let chars: Vec<char> = line.chars().collect();
        let value =
            isize::from_str_radix(chars[1..].iter().collect::<String>().as_str(), 10).unwrap();
        match chars[0] {
            'N' => waypoint_y += value,
            'S' => waypoint_y -= value,
            'E' => waypoint_x += value,
            'W' => waypoint_x -= value,
            'R' => {
                (waypoint_x, waypoint_y) = rotate(value, waypoint_x, waypoint_y);
            }
            'L' => {
                (waypoint_x, waypoint_y) = rotate(-value, waypoint_x, waypoint_y);
            }
            'F' => {
                (ship_x, ship_y) = move_forward(value, ship_x, ship_y, waypoint_x, waypoint_y);
            }
            _ => (),
        }
    }

    ship_x.abs() + ship_y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 25);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 286);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
