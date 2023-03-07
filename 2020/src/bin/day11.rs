use std::collections::HashSet;
use std::fs;

const DAY: u32 = 11;

fn parse_seat_map(text: &str) -> (HashSet<(i64, i64)>, i64, i64) {
    let mut seat_map: HashSet<(i64, i64)> = HashSet::new();
    let mut row_num: i64 = 0;
    let mut col_num: i64 = 0;
    for (row, line) in text.lines().enumerate() {
        row_num += 1;
        col_num = line.len() as i64;
        for (col, point) in line.chars().enumerate() {
            if point == 'L' {
                seat_map.insert((row as i64, col as i64));
            }
        }
    }
    (seat_map, row_num, col_num)
}

fn adjacent_occupied_seats(row: i64, col: i64, occupied_seats: &HashSet<(i64, i64)>) -> i64 {
    let mut occupied_seats_count: i64 = 0;
    let change_values = [-1, 0, 1];
    for drow in change_values {
        for dcol in change_values {
            if !(drow == 0 && dcol == 0) {
                let new_row = row + drow;
                let new_col = col + dcol;
                if occupied_seats.contains(&(new_row, new_col)) {
                    occupied_seats_count += 1;
                }
            }
        }
    }
    occupied_seats_count
}

fn visibled_occupied_seats(
    row: i64,
    col: i64,
    row_num: i64,
    col_num: i64,
    occupied_seats: &HashSet<(i64, i64)>,
    empty_seats: &HashSet<(i64, i64)>,
) -> i64 {
    let mut occupied_seats_count: i64 = 0;
    let change_values = [-1, 0, 1];
    let mut new_row = row;
    let mut new_col = col;
    for drow in change_values {
        for dcol in change_values {
            if !(drow == 0 && dcol == 0) {
                loop {
                    new_row = new_row + drow;
                    new_col = new_col + dcol;
                    if new_row < 0 || new_row >= row_num || new_col < 0 || new_col >= col_num {
                        new_row = row;
                        new_col = col;
                        break;
                    }
                    if occupied_seats.contains(&(new_row, new_col)) {
                        occupied_seats_count += 1;
                        new_row = row;
                        new_col = col;
                        break;
                    }
                    if empty_seats.contains(&(new_row, new_col)) {
                        new_row = row;
                        new_col = col;
                        break;
                    }
                }
            }
        }
    }
    occupied_seats_count
}

fn print_seats(
    row_num: i64,
    col_num: i64,
    empty_seats: &HashSet<(i64, i64)>,
    occupied_seats: &HashSet<(i64, i64)>,
) {
    for row in 0..row_num {
        for col in 0..col_num {
            let location = (row, col);
            if empty_seats.contains(&location) {
                print!("L");
            } else if occupied_seats.contains(&location) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn simulate(
    empty_seats: &HashSet<(i64, i64)>,
    occupied_seats: &HashSet<(i64, i64)>,
) -> (HashSet<(i64, i64)>, HashSet<(i64, i64)>) {
    let mut new_empty_seats: HashSet<(i64, i64)> = empty_seats.clone();
    let mut new_occupied_seats: HashSet<(i64, i64)> = occupied_seats.clone();
    for (row, col) in empty_seats.iter() {
        if adjacent_occupied_seats(*row, *col, &occupied_seats) == 0 {
            new_occupied_seats.insert((*row, *col));
            new_empty_seats.remove(&(*row, *col));
        }
    }
    for (row, col) in occupied_seats.iter() {
        if adjacent_occupied_seats(*row, *col, &occupied_seats) >= 4 {
            new_empty_seats.insert((*row, *col));
            new_occupied_seats.remove(&(*row, *col));
        }
    }
    // print_seats(row_num, col_num, &new_empty_seats, &new_occupied_seats);
    (new_empty_seats, new_occupied_seats)
}

fn part_one(text: &str) -> usize {
    let (mut empty_seats, _, _) = parse_seat_map(text);
    let mut occupied_seats: HashSet<(i64, i64)> = HashSet::new();
    loop {
        let (new_empty_seats, new_occupied_seats) = simulate(&empty_seats, &occupied_seats);
        if new_empty_seats == empty_seats && new_occupied_seats == occupied_seats {
            return new_occupied_seats.len();
        }
        empty_seats = new_empty_seats;
        occupied_seats = new_occupied_seats;
    }
}

fn simulate_two(
    row_num: i64,
    col_num: i64,
    empty_seats: &HashSet<(i64, i64)>,
    occupied_seats: &HashSet<(i64, i64)>,
) -> (HashSet<(i64, i64)>, HashSet<(i64, i64)>) {
    let mut new_empty_seats: HashSet<(i64, i64)> = empty_seats.clone();
    let mut new_occupied_seats: HashSet<(i64, i64)> = occupied_seats.clone();
    for (row, col) in empty_seats.iter() {
        if visibled_occupied_seats(*row, *col, row_num, col_num, &occupied_seats, &empty_seats) == 0
        {
            new_occupied_seats.insert((*row, *col));
            new_empty_seats.remove(&(*row, *col));
        }
    }
    for (row, col) in occupied_seats.iter() {
        if visibled_occupied_seats(*row, *col, row_num, col_num, &occupied_seats, &empty_seats) >= 5
        {
            new_empty_seats.insert((*row, *col));
            new_occupied_seats.remove(&(*row, *col));
        }
    }
    // print_seats(row_num, col_num, &new_empty_seats, &new_occupied_seats);
    (new_empty_seats, new_occupied_seats)
}

fn part_two(text: &str) -> usize {
    let (mut empty_seats, row_num, col_num) = parse_seat_map(text);
    let mut occupied_seats: HashSet<(i64, i64)> = HashSet::new();
    loop {
        let (new_empty_seats, new_occupied_seats) =
            simulate_two(row_num, col_num, &empty_seats, &occupied_seats);
        if new_empty_seats == empty_seats && new_occupied_seats == occupied_seats {
            return new_occupied_seats.len();
        }
        empty_seats = new_empty_seats;
        occupied_seats = new_occupied_seats;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_one(&lines), 37);
    }

    #[test]
    fn test_part_two() {
        let lines =
            fs::read_to_string(format!("input/day{DAY}_test_input")).expect("Can't read file");
        assert_eq!(part_two(&lines), 26);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
