use std::fs;

fn part_one(text: &str) -> i64 {
    let histories: Vec<Vec<i64>> = text
        .lines()
        .map(|l| l.split_whitespace().map(|e| e.parse().unwrap()).collect())
        .collect();
    let mut predictions: Vec<i64> = vec![];
    for history in &histories {
        let mut diffs = vec![history.clone()];
        loop {
            let current_diff = diffs.last().unwrap();
            let next_diff: Vec<i64> = current_diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            diffs.push(next_diff.clone());
            if next_diff.iter().all(|&e| e == 0) {
                break;
            }
        }
        for i in (1..diffs.len()).rev() {
            let a = *diffs[i - 1].last().unwrap();
            let b = *diffs[i].last().unwrap();
            diffs[i - 1].push(a + b);
        }

        predictions.push(*diffs[0].last().unwrap());
    }

    predictions.iter().sum()
}

fn part_two(text: &str) -> i64 {
    let histories: Vec<Vec<i64>> = text
        .lines()
        .map(|l| l.split_whitespace().map(|e| e.parse().unwrap()).collect())
        .collect();
    let mut predictions: Vec<i64> = vec![];
    for history in &histories {
        let mut diffs = vec![history.clone()];
        loop {
            let current_diff = diffs.last().unwrap();
            let next_diff: Vec<i64> = current_diff
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect();
            diffs.push(next_diff.clone());
            if next_diff.iter().all(|&e| e == 0) {
                break;
            }
        }
        for i in (1..diffs.len()).rev() {
            let a = *diffs[i - 1].first().unwrap();
            let b = *diffs[i].first().unwrap();
            diffs[i - 1].insert(0, a - b);
        }
        predictions.push(*diffs[0].first().unwrap());
    }

    predictions.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 2);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day9").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
