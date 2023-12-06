use std::fs;

fn part_one(text: &str) -> u64 {
    let mut times: Vec<f64> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();

    for line in text.lines() {
        if let Some(time_str) = line.strip_prefix("Time:") {
            times = time_str.split_whitespace().map(|e| e.parse().unwrap()).collect();
        } else if let Some(distance_str) = line.strip_prefix("Distance:") {
            distances = distance_str.split_whitespace().map(|e| e.parse().unwrap()).collect();
        }
    }

    let mut chooses: Vec<u64> = Vec::new();
    for (time, distance) in times.iter().zip(distances.iter()) {
        let temp = (time.powi(2) - 4_f64 * distance).sqrt();
        let mut left = ((time - temp) / 2_f64).ceil();
        let mut right = ((time + temp) / 2_f64).floor();
        if (time - temp) % 2_f64 == 0_f64 {
            left += 1_f64;
            right -= 1_f64;
        }
        chooses.push((right - left + 1_f64) as u64);
    }

    chooses.iter().product()
}

// fn part_two(text: &str) -> u64 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 288);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day6").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
