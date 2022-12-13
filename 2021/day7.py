import unittest
# https://libreddit.domain.glass/r/adventofcode/comments/rawxad
# https://github.com/mebeim/aoc/tree/master/2021#day-7---the-treachery-of-whales


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        positions = list(map(int, f.readline().split(",")))
        positions.sort()
        median = positions[len(positions) // 2]
        return sum(abs(v - median) for v in positions)


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        positions = list(map(int, f.readline().split(",")))
        mean = sum(positions) // len(positions)
        return min(part_two_fuel(positions, mean), part_two_fuel(positions, mean + 1))


def part_two_fuel(positions: list[int], align_position) -> int:
    fuel = 0
    for position in positions:
        distance = abs(position - align_position)
        fuel += (distance * (distance + 1)) // 2
    return fuel


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day7_test_input"), 37)

    def test_part_two(self):
        self.assertEqual(part_two("input/day7_test_input"), 168)


if __name__ == "__main__":
    print(part_one("input/day7_input"))
    print(part_two("input/day7_input"))
