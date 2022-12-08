use std::fs;

pub fn resolve(part: u32) {
    let lines = fs::read_to_string("src/day7_input").expect("Can't read file");

    match part {
        1 => println!("{}", part_one(lines)),
        2 => println!("{}", part_two(lines)),
        _ => (),
    }
}

enum DirectoryContent {
    File(NormalFile),
    Dir(Directory),
}

struct NormalFile {
    name: String,
    size: u32,
}

struct Directory {
    name: String,
    contents: Vec<DirectoryContent>,
}

fn create_fs_tree(text: String) -> Directory {
    let mut pwd: Vec<&mut Directory> = vec![];
    let mut root: Directory = Directory {
        name: String::from("/"),
        contents: vec![],
    };
    pwd.push(&mut root);
    for line in text.lines() {
        let output: Vec<&str> = line.split_whitespace().collect();
        match output[0] {
            "$" => {
                if output[1] == "cd" {
                    if output[2] == ".." {
                        pwd.pop();
                    } else if !pwd.is_empty() {
                        for d in pwd.last().unwrap().contents.iter() {
                            match d {
                                DirectoryContent::Dir(dir) => {
                                    if dir.name == output[2] {
                                        pwd.push(dir);  // damn
                                        break;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
            "dir" => {
                pwd.last()
                    .unwrap()
                    .contents
                    .push(DirectoryContent::Dir(Directory {
                        name: String::from(output[1]),
                        contents: vec![],
                    }));
            }
            _ => {
                pwd.last()
                    .unwrap()
                    .contents
                    .push(DirectoryContent::File(NormalFile {
                        name: String::from(output[1]),
                        size: output[2].parse().unwrap(),
                    }));
            }
        }
    }

    root
}

fn calculate_folder_size(folder: &Directory, folder_sizes: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    for content in folder.contents.iter() {
        match content {
            DirectoryContent::File(f) => size += f.size,
            DirectoryContent::Dir(d) => size += calculate_folder_size(d, folder_sizes),
        }
    }

    folder_sizes.push(size);
    size
}

fn part_one(text: String) -> u32 {
    let root = create_fs_tree(text);
    let mut folder_sizes: Vec<u32> = vec![];
    calculate_folder_size(&root, &mut folder_sizes);

    folder_sizes.iter().filter(|v| **v <= 100000).sum()
}

fn part_two(text: String) -> u32 {
    let root = create_fs_tree(text);
    let mut folder_sizes: Vec<u32> = vec![];
    calculate_folder_size(&root, &mut folder_sizes);
    folder_sizes.sort();
    let total_disk_size = 70000000;
    let root_size = folder_sizes.last().unwrap();
    let free_size = total_disk_size - root_size;
    for size in folder_sizes.iter() {
        if size + free_size >= 30000000 {
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
