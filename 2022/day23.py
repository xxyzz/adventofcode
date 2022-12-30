import unittest
import math
from collections import defaultdict

DAY = 23

DIRECTIONS = [[(-1, 0), (-1, -1), (-1, 1)], # N, NW, NE
              [(1, 0), (1, -1), (1, 1)],    # S, SW, SE
              [(0, -1), (-1, -1), (1, -1)], # W, NW, SW
              [(0, 1), (-1, 1), (1, 1)]]    # E, NE, SE


def parse_map(path: str) -> set[tuple[int, int]]:
    with open(path) as f:
        eleves_map = set()
        for row, line in enumerate(f):
            for col, value in enumerate(line.strip()):
                if value == "#":
                    eleves_map.add((row, col))
        return eleves_map

def adjancent_positions(row, col):
    for d_r, d_c in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
        yield row + d_r, col + d_c

def part_one(path: str) -> int:
    eleves_map = parse_map(path)
    eleves_num = len(eleves_map)
    # print_map(eleves_map)
    # print()
    for round_count in range(10):
        new_map = {loc: loc for loc in eleves_map}
        new_loc_count: dict[tuple[int, int], int] = defaultdict(int)
        for row, col in eleves_map:
            new_loc = (row, col)
            if all(loc not in eleves_map for loc in adjancent_positions(row, col)):
                new_loc_count[new_loc] += 1
                continue
            for i in range(4):
                if all((row + dr, col + dc) not in eleves_map for dr, dc in DIRECTIONS[(round_count + i) % 4]):
                    new_loc = row + DIRECTIONS[(round_count + i) % 4][0][0], col + DIRECTIONS[(round_count + i) % 4][0][1]
                    break
            new_map[(row, col)] = new_loc
            new_loc_count[new_loc] += 1

        for origin_loc, new_loc in new_map.items():
            if new_loc_count[new_loc] > 1:
                new_map[origin_loc] = origin_loc
        eleves_map = set(new_map.values())

        # print(f"round {round_count + 1}")
        # print_map(eleves_map)
        # print()

    min_row = math.inf
    max_row = -math.inf
    min_col = math.inf
    max_col = -math.inf
    for row, col in eleves_map:
        min_row = min(row, min_row)
        max_row = max(row, max_row)
        min_col = min(col, min_col)
        max_col = max(col, max_col)

    return (max_col - min_col + 1) * (max_row - min_row + 1) - len(eleves_map)


def print_map(eleves_map):
    for row in range(14):
        for col in range(14):
            if (row, col) in eleves_map:
                print("#", end="")
            else:
                print(".", end="")
        print()

def part_two(input_path: str) -> int:
    pass


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 110)

    # def test_part_one1(self):
    #     self.assertEqual(part_one(f"input/day{DAY}_test_input1"), 110)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
