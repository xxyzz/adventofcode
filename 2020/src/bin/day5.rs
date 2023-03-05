use std::fs;

const DAY: u32 = 5;

fn get_seat_id(boarding_pass: &str) -> u32 {
    let seat_letters: Vec<char> = boarding_pass.chars().collect();
    let row_num = find_row(&seat_letters[..7]);
    let col_num = find_column(&seat_letters[7..]);
    row_num * 8 + col_num
}

fn find_row(letters: &[char]) -> u32 {
    let mut lower_range = 0;
    let mut heigher_range = 127;
    for letter in letters {
        let mid_point = (lower_range + heigher_range) / 2;
        if *letter == 'F' {
            heigher_range = mid_point;
        } else if *letter == 'B' {
            lower_range = mid_point + 1;
        }
    }
    if *letters.last().unwrap() == 'F' {
        heigher_range
    } else {
        lower_range
    }
}

fn find_column(letters: &[char]) -> u32 {
    let mut lower_range = 0;
    let mut heigher_range = 7;
    for letter in letters {
        let mid_point = (lower_range + heigher_range) / 2;
        if *letter == 'L' {
            heigher_range = mid_point;
        } else if *letter == 'R' {
            lower_range = mid_point + 1;
        }
    }
    if *letters.last().unwrap() == 'R' {
        heigher_range
    } else {
        lower_range
    }
}

fn part_one(text: &str) -> u32 {
    text.lines().map(|line| get_seat_id(line)).max().unwrap()
}

fn part_two(text: &str) -> u32 {
    let mut seat_ids: Vec<u32> = text.lines().map(|line| get_seat_id(line)).collect();
    seat_ids.sort();
    for index in 0..seat_ids.len() - 1 {
        if seat_ids[index + 1] - seat_ids[index] == 2 {
            return seat_ids[index] + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("FBFBBFFRLR"), 357);
        assert_eq!(part_one("BFFFBBFRRR"), 567);
        assert_eq!(part_one("FFFBBBFRRR"), 119);
        assert_eq!(part_one("BBFFBBFRLL"), 820);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}_input")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
