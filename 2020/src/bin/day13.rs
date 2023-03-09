use std::fs;

const DAY: u32 = 13;

// https://github.com/norvig/pytudes/blob/main/ipynb/Advent-2020.ipynb
fn wait_time(bus_depart_time: usize, time: usize) -> usize {
    if time % bus_depart_time == 0 {
	0
    } else {
	bus_depart_time - time % bus_depart_time
    }
}

fn part_one(text: &str) -> usize {
    let lines: Vec<&str> = text.lines().collect();
    let start_time = usize::from_str_radix(lines[0], 10).unwrap();
    let bus = lines[1]
        .split(",")
        .filter(|&b| b != "x")
        .map(|b| usize::from_str_radix(b, 10).unwrap())
        .min_by_key(|&b| wait_time(b, start_time))
        .unwrap();
    bus * wait_time(bus, start_time)
}

fn part_two(text: &str) -> usize {
    let lines: Vec<&str> = text.lines().collect();
    let buses: Vec<(usize, usize)> = lines[1]
        .split(",")
        .enumerate()
        .filter(|(_, b)| *b != "x")
        .map(|(i, b)| (i, usize::from_str_radix(b, 10).unwrap()))
        .collect();
    let mut time = 0;
    let mut step = 1;
    for (index, depart_time) in buses.iter() {
        while (time + index) % depart_time != 0 {
	    time += step;
	}
	step *= depart_time;
    }
    time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
        assert_eq!(part_one(&lines), 295);
    }

    #[test]
    fn test_part_two() {
        let lines = fs::read_to_string(format!("input/day{DAY}_test")).expect("Can't read file");
        assert_eq!(part_two(&lines), 1068781);
    }
}

fn main() {
    let lines = fs::read_to_string(format!("input/day{DAY}")).expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
