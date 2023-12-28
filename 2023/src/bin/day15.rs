use std::{
    collections::{HashMap, VecDeque},
    fs,
    str::Chars,
};

fn hash(chars: Chars) -> u64 {
    let mut current_value = 0;
    for c in chars {
        current_value += c as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn part_one(text: &str) -> u64 {
    if let Some(line) = text.lines().next() {
        return line.split(',').map(|step| hash(step.chars())).sum();
    }
    0
}

struct LensHashMap {
    len_boxes: HashMap<usize, VecDeque<(String, usize)>>,
}

impl LensHashMap {
    fn process_step(&mut self, step: &str) {
        if step.contains('=') {
            let split_str: Vec<&str> = step.split('=').collect();
            let marker = split_str[0];
            let focal_length: usize = split_str[1].parse().unwrap();
            self.insert(marker, focal_length);
        } else if step.contains('-') {
            let split_str: Vec<&str> = step.split('-').collect();
            let marker = split_str[0];
            self.remove(marker);
        }
    }

    fn insert(&mut self, new_marker: &str, focal_len: usize) {
        let box_index = hash(new_marker.chars()) as usize;
        if !self.len_boxes.contains_key(&box_index) {
            self.len_boxes.insert(box_index, VecDeque::new());
        }
        let mut inserted = false;
        let lens = self.len_boxes.get_mut(&box_index).unwrap();
        for (index, (marker, _)) in lens.clone().iter().enumerate() {
            if *marker == new_marker {
                inserted = true;
                lens[index] = (new_marker.to_string(), focal_len);
            }
        }
        if !inserted {
            lens.push_back((new_marker.to_string(), focal_len));
        }
    }

    fn remove(&mut self, new_marker: &str) {
        let box_index = hash(new_marker.chars()) as usize;
        if self.len_boxes.contains_key(&box_index) {
            let lens = self.len_boxes.get_mut(&box_index).unwrap();
            for (index, (marker, _)) in lens.clone().iter().enumerate() {
                if *marker == new_marker {
                    lens.remove(index);
                }
            }
        }
    }

    fn compute_focus_power(&self) -> usize {
        self.len_boxes
            .iter()
            .map(|(box_index, lens)| {
                lens.iter()
                    .enumerate()
                    .map(|(len_index, (_, focal_len))| {
                        (box_index + 1) * (len_index + 1) * focal_len
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn part_two(text: &str) -> usize {
    let mut len_hm = LensHashMap {
        len_boxes: HashMap::new(),
    };
    if let Some(line) = text.lines().next() {
        line.split(',').for_each(|step| len_hm.process_step(step));
        return len_hm.compute_focus_power();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 1320);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 145);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day15").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
