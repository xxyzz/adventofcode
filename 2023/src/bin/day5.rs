use std::fs;

#[derive(Clone, Debug)]
struct Map {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

fn part_one(text: &str) -> u64 {
    let mut map_lists: Vec<Vec<Map>> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    let mut current_maps: Vec<Map> = Vec::new();

    for line in text.lines() {
        if let Some(seeds_str) = line.strip_prefix("seeds: ") {
            seeds = seeds_str.split(' ').map(|e| e.parse().unwrap()).collect();
        } else if !line.trim().is_empty() && !line.ends_with(':') {
            let nums: Vec<u64> = line.split(' ').map(|e| e.parse().unwrap()).collect();
            current_maps.push(Map {
                dest_start: nums[0],
                source_start: nums[1],
                length: nums[2],
            });
        } else if line.trim().is_empty() && !current_maps.is_empty() {
            map_lists.push(current_maps.clone());
            current_maps.clear();
        }
    }
    map_lists.push(current_maps.clone());

    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut location = seed;
        for map_list in &map_lists {
            for map in map_list {
                if location >= map.source_start && location < map.source_start + map.length {
                    location = map.dest_start + location - map.source_start;
                    break;
                }
            }
        }
        if location < min_location {
            min_location = location;
        }
    }

    min_location
}

// fn part_two(text: &str) -> u32 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 35);
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), );
    // }
}

fn main() {
    let lines = fs::read_to_string("input/day5").expect("Can't read file");
    println!("{}", part_one(&lines));
    // println!("{}", part_two(&lines));
}
