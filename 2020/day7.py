import re
import unittest
from collections import defaultdict

DAY = 7
# https://github.com/sophiebits/adventofcode/blob/main/2020/day07.py

def parse_bags(input_path: str) -> dict[str, set[str]]:
    with open(input_path) as f:
        bags_dict = defaultdict(set)
        for line in f:
            bags = re.findall(r"([a-z ]+) bags?", line)
            for bag in bags[1:]:
                bags_dict[bag.strip()].add(bags[0].strip())
        return bags_dict


def find_bags(bags_dict: dict[str, set[str]], target_bag: str) -> int:
    bags: set[str] = set()

    def find(bag: str) -> None:
        for contained_by in bags_dict[bag]:
            bags.add(contained_by)
            find(contained_by)

    find(target_bag)
    return len(bags)


def part_one(input_path: str) -> int:
    return find_bags(parse_bags(input_path), "shiny gold")


def parse_bags_num(input_path: str) -> dict[str, list[tuple[int, str]]]:
    with open(input_path) as f:
        bags_dict = defaultdict(list)
        for line in f:
            bags = re.findall(r"(\d+ )?([a-z ]+) bags?", line)
            for count, bag in bags[1:]:
                bags_dict[bags[0][1]].append((int(count), bag))
        return bags_dict


def count_bags(bags_dict: dict[str, list[tuple[int, str]]], target_bag: str) -> int:
    bags_num = 0
    for count, contained_name in bags_dict[target_bag]:
        bags_num += count
        bags_num += count * count_bags(bags_dict, contained_name)
    return bags_num


def part_two(input_path: str) -> int:
    return count_bags(parse_bags_num(input_path), "shiny gold")


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 4)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 32)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
