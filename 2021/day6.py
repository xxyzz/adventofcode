import unittest
from collections import Counter, defaultdict
from typing import TextIO


def parse_fish(test_input: str) -> dict[int, int]:
    with open(test_input) as f:
        return Counter(map(int, f.read().strip().split(",")))


def run(fish: dict[int, int], days: int) -> tuple[dict[int, int], int]:
    for _ in range(days):
        new_fish: dict[int, int] = defaultdict(int)
        for timer, count in fish.items():
            if timer == 0:
                new_fish[6] += count
                new_fish[8] += count
            else:
                new_fish[timer - 1] += count
        fish = new_fish

    return fish, sum(fish.values())


class Test(unittest.TestCase):
    def test_part_one(self):
        fish = parse_fish("input/day6_test_input")
        _, fish_count = run(fish, 80)
        self.assertEqual(fish_count, 5934)

    def test_part_two(self):
        fish = parse_fish("input/day6_test_input")
        _, fish_count = run(fish, 256)
        self.assertEqual(fish_count, 26984457539)


if __name__ == "__main__":
    fish = parse_fish("input/day6_input")
    fish, fish_count = run(fish, 80)
    print(fish_count)
    _, fish_count = run(fish, 256 - 80)
    print(fish_count)
