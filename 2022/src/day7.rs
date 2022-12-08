use std::collections::HashMap;
use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day7_input").expect("Can't read file");

    match part {
        1 => println!("{}", part_one(lines)),
        2 => println!("{}", part_two(lines)),
        _ => (),
    }
}

fn add_file_sizes(text: String) -> Vec<u32> {
    let mut pwd: Vec<&str> = vec![];
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    for line in text.lines() {
        let output: Vec<&str> = line.split_whitespace().collect();
        if line.starts_with("$ cd") {
            if line == "$ cd .." {
                let current_dir = pwd.join("/");
                pwd.pop();
                let current_dir_size = *dir_sizes.get(&current_dir).unwrap();
                let parent_dir = pwd.join("/");
                dir_sizes
                    .entry(String::from(parent_dir))
                    .and_modify(|size| *size += current_dir_size);
            } else {
                pwd.push(output[2]);
                let current_dir = pwd.join("/");
                dir_sizes.insert(current_dir, 0);
            }
        } else if !line.starts_with("$") && !line.starts_with("dir") {
            let filesize: u32 = output[0].parse().unwrap();
            let dir = pwd.join("/");
            dir_sizes
                .entry(String::from(dir))
                .and_modify(|size| *size += filesize);
        }
    }

    for _ in 0..pwd.len() - 1 {
        let current_dir = pwd.join("/");
        pwd.pop();
        let parent_dir = pwd.join("/");
        let current_dir_size = *dir_sizes.get(&current_dir).unwrap();
        dir_sizes
            .entry(parent_dir)
            .and_modify(|size| *size += current_dir_size);
    }

    dir_sizes.into_values().collect()
}

fn part_one(text: String) -> u32 {
    let dir_sizes = add_file_sizes(text);
    dir_sizes.iter().filter(|v| **v <= 100000).sum()
}

fn part_two(text: String) -> u32 {
    let mut dir_sizes = add_file_sizes(text);
    dir_sizes.sort();
    let total_disk_size = 70000000;
    let root_size = *dir_sizes.last().unwrap();
    let free_size = total_disk_size - root_size;
    for size in dir_sizes.iter() {
        if *size + free_size >= 30000000 {
            return *size;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_input = fs::read_to_string("src/day7_test_input").expect("Can't read file");
        assert_eq!(part_one(test_input), 95437);
    }

    #[test]
    fn test_part_two() {
        let test_input = fs::read_to_string("src/day7_test_input").expect("Can't read file");
        assert_eq!(part_two(test_input), 24933642);
    }
}
