use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Debug)]
struct Map {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

fn parse_input(text: &str) -> (Vec<u64>, Vec<Vec<Map>>) {
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
    (seeds, map_lists)
}

fn part_one(text: &str) -> u64 {
    let (seeds, map_lists) = parse_input(text);
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

#[derive(Clone, Debug)]
struct Range {
    start: u64,
    end: u64,
}

fn apply_map(range: &Range, map: &Map) -> Option<(Range, Vec<Range>)> {
    if range.start < map.source_start + map.length && range.end > map.source_start {
        let interset_left = max(range.start, map.source_start);
        let interset_right = min(range.end, map.source_start + map.length);
        let mut leftovers = Vec::new();
        let mapped_range = Range {
            start: map.dest_start + interset_left - map.source_start,
            end: map.dest_start + interset_right - map.source_start,
        };
        if range.start < map.source_start {
            leftovers.push(Range {
                start: range.start,
                end: interset_left,
            });
        }
        if range.end > map.source_start + map.length {
            leftovers.push(Range {
                start: interset_right,
                end: range.end,
            });
        }
        return Some((mapped_range, leftovers));
    }
    None
}

fn part_two(text: &str) -> u64 {
    let (seed_nums, map_lists) = parse_input(text);
    let mut min_location = u64::MAX;

    for (&seed_start, &seed_range) in seed_nums
        .iter()
        .step_by(2)
        .zip(seed_nums.iter().skip(1).step_by(2))
    {
        let mut unmapped_ranges = VecDeque::from([Range {
            start: seed_start,
            end: seed_start + seed_range,
        }]);
        let mut mapped_ranges = VecDeque::new();
        for map_list in &map_lists {
            mapped_ranges.clear();

            while !unmapped_ranges.is_empty() {
                let range = unmapped_ranges.pop_front().unwrap();
                let mut has_interset = false;
                for map in map_list {
                    if let Some((mapped_range, leftovers)) = apply_map(&range, map) {
                        mapped_ranges.push_back(mapped_range);
                        unmapped_ranges.extend(leftovers);
                        has_interset = true;
                        break;
                    }
                }
                if !has_interset {
                    mapped_ranges.push_back(range);
                }
            }
            unmapped_ranges = mapped_ranges.clone();
        }
        min_location = min(
            min_location,
            mapped_ranges.iter().map(|e| e.start).min().unwrap(),
        );
    }
    min_location
}

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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 46);
    }
}

fn main() {
    let lines = fs::read_to_string("input/day5").expect("Can't read file");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}
