import math
import unittest
from collections import deque

DAY = 9


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        heightmap = [list(map(int, line.strip())) for line in f]
        row_num = len(heightmap)
        col_num = len(heightmap[0])
        result = 0
        for row in range(row_num):
            for col in range(col_num):
                current_height = heightmap[row][col]
                if all(
                    current_height < adjacent_location
                    for adjacent_location, _, _ in adjacent_locations(
                        heightmap, row, col
                    )
                ):
                    result += current_height + 1
        return result


def adjacent_locations(heightmap, row, col):
    row_num = len(heightmap)
    col_num = len(heightmap[0])
    for d_r, d_c in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
        new_r = row + d_r
        new_c = col + d_c
        if 0 <= new_r < row_num and 0 <= new_c < col_num:
            yield heightmap[new_r][new_c], new_r, new_c


def bfs(heightmap, row, col):
    q = deque()
    q.append((row, col))
    visited = set()

    while q:
        r, c = q.popleft()
        visited.add((r, c))
        for height, nr, nc in adjacent_locations(heightmap, r, c):
            if (nr, nc) not in visited and height != 9:
                q.append((nr, nc))
    return visited


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        heightmap = [list(map(int, line.strip())) for line in f]
        row_num = len(heightmap)
        col_num = len(heightmap[0])
        basins = []
        for row in range(row_num):
            for col in range(col_num):
                current_height = heightmap[row][col]
                if all(
                    current_height < adjacent_location
                    for adjacent_location, _, _ in adjacent_locations(
                        heightmap, row, col
                    )
                ):
                    basins.append(bfs(heightmap, row, col))
        basin_sizes = [len(basin) for basin in basins]
        basin_sizes.sort(reverse=True)
        return math.prod(basin_sizes[:3])


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 15)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 1134)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
