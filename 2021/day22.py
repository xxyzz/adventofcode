import functools
import itertools
import re
import unittest
from collections import defaultdict

DAY = 22


def parse_input(path: str) -> list[tuple[bool, list[int]]]:
    steps = []
    with open(path) as f:
        for line in f:
            steps.append(
                (line.startswith("on"), list(map(int, re.findall(r"-?\d+", line))))
            )
    return steps


def part_one(input_path: str) -> int:
    on_cubes = set()
    steps = parse_input(input_path)
    for turn_on, (min_x, max_x, min_y, max_y, min_z, max_z) in steps:
        if (
            min_x < -50
            or max_x > 50
            or min_y < -50
            or max_y > 50
            or min_z < -50
            or max_z > 50
        ):
            continue
        cubes = set(
            itertools.product(
                range(min_x, max_x + 1),
                range(min_y, max_y + 1),
                range(min_z, max_z + 1),
            )
        )
        if turn_on:
            on_cubes |= cubes
        else:
            on_cubes -= cubes
    return len(on_cubes)


# https://libreddit.domain.glass/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/hpizza8
def part_two(input_path: str) -> int:
    steps = parse_input(input_path)
    ranges: dict[tuple[int, int, int, int, int, int], int] = defaultdict(int)
    for turn_on, step_ranges in steps:
        min_x, max_x, min_y, max_y, min_z, max_z = step_ranges
        new_ranges = ranges.copy()
        for (
            emin_x,
            emax_x,
            emin_y,
            emax_y,
            emin_z,
            emax_z,
        ), ecount in ranges.items():  # exist
            imin_x = max(min_x, emin_x)  # intersect
            imax_x = min(max_x, emax_x)
            imin_y = max(min_y, emin_y)
            imax_y = min(max_y, emax_y)
            imin_z = max(min_z, emin_z)
            imax_z = min(max_z, emax_z)

            if imin_x <= imax_x and imin_y <= imax_y and imin_z <= imax_z:
                key = (imin_x, imax_x, imin_y, imax_y, imin_z, imax_z)
                new_ranges[key] -= ecount
                if new_ranges[key] == 0:
                    del new_ranges[key]
        if turn_on:
            new_ranges[tuple(step_ranges)] += 1
        ranges = new_ranges

    return sum(
        (max_x - min_x + 1) * (max_y - min_y + 1) * (max_z - min_z + 1) * count
        for (min_x, max_x, min_y, max_y, min_z, max_z), count in ranges.items()
    )


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 590784)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input1"), 2758514936282235)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
