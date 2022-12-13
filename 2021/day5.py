import unittest
from collections import defaultdict


def part_one(test_input: str | None = None) -> int:
    point_counts = defaultdict(int)
    with open(test_input if test_input else "input/day5_input") as f:
        for line in f:
            point1_str, point2_str = line.strip().split(" -> ")
            point1_x, point1_y = [int(v) for v in point1_str.split(",")]
            point2_x, point2_y = [int(v) for v in point2_str.split(",")]
            if point1_x == point2_x:
                if point1_y > point2_y:
                    point1_y, point2_y = point2_y, point1_y
                for y in range(point1_y, point2_y + 1):
                    point_counts[(point1_x, y)] += 1
            elif point1_y == point2_y:
                if point1_x > point2_x:
                    point1_x, point2_x = point2_x, point1_x
                for x in range(point1_x, point2_x + 1):
                    point_counts[(x, point1_y)] += 1
        return sum(v >= 2 for v in point_counts.values())


def part_two(test_input: str | None = None) -> int:
    with open(test_input if test_input else "input/day5_input") as f:
        point_counts = defaultdict(int)
        with open(test_input if test_input else "input/day5_input") as f:
            for line in f:
                point1_str, point2_str = line.strip().split(" -> ")
                point1_x, point1_y = [int(v) for v in point1_str.split(",")]
                point2_x, point2_y = [int(v) for v in point2_str.split(",")]
                if point1_x == point2_x:
                    if point1_y > point2_y:
                        point1_y, point2_y = point2_y, point1_y
                    for y in range(point1_y, point2_y + 1):
                        point_counts[(point1_x, y)] += 1
                elif point1_y == point2_y:
                    if point1_x > point2_x:
                        point1_x, point2_x = point2_x, point1_x
                    for x in range(point1_x, point2_x + 1):
                        point_counts[(x, point1_y)] += 1
                else:
                    dy = point1_y - point2_y
                    dx = point1_x - point2_x
                    slope = dy / dx
                    if abs(slope) == 1:
                        b = point1_y - slope * point1_x
                        if point1_x > point2_x:
                            point1_x, point2_x = point2_x, point1_x
                            point1_y, point2_y = point2_y, point1_y
                        for x in range(point1_x, point2_x + 1):
                            y = slope * x + b
                            point_counts[(x, y)] += 1
        return sum(v >= 2 for v in point_counts.values())


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day5_test_input"), 5)

    def test_part_two(self):
        self.assertEqual(part_two("input/day5_test_input"), 12)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
