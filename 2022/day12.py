import unittest
from collections import deque
from collections.abc import Callable, Generator
from typing import TextIO


def parse_heightmap(
    f: TextIO,
) -> tuple[list[list[str]], tuple[int, int], tuple[int, int]]:
    heightmap = [[c for c in line.strip()] for line in f]
    row_num = len(heightmap)
    col_num = len(heightmap[0])
    start_point = (-1, -1)
    end_point = (-1, -1)
    for row in range(row_num):
        for col in range(col_num):
            if heightmap[row][col] == "S":
                heightmap[row][col] = "a"
                start_point = (row, col)
            elif heightmap[row][col] == "E":
                heightmap[row][col] = "z"
                end_point = (row, col)
            if start_point != (-1, -1) and end_point != (-1, -1):
                break
    return heightmap, start_point, end_point


def dfs(
    heightmap: list[list[str]],
    start_point: tuple[int, int],
    dest: str
    | tuple[int, int],  # point or height character, a heightmap could have several "z"
    get_neighbers: Callable[
        [list[list[str]], tuple[int, int]], Generator[tuple[int, int], None, None]
    ],
) -> int:
    visited_points: set[tuple[int, int]] = set()
    q: deque[tuple[int, tuple[int, int]]] = deque()
    q.append((0, start_point))
    while q:
        steps, point = q.popleft()
        row, col = point
        if point == dest or heightmap[row][col] == dest:
            return steps

        if point not in visited_points:
            visited_points.add(point)

            for neighber in get_neighbers(heightmap, point):
                if neighber in visited_points:
                    continue
                q.append((steps + 1, neighber))

    return -1


def get_four_neighbers(
    row: int, col: int, row_num: int, col_num: int
) -> Generator[tuple[int, int], None, None]:
    for n_row, n_col in [
        [row - 1, col],
        [row + 1, col],
        [row, col - 1],
        [row, col + 1],
    ]:
        if 0 <= n_row < row_num and 0 <= n_col < col_num:
            yield n_row, n_col


def get_neighbers_forward(
    heightmap: list[list[str]], point: tuple[int, int]
) -> Generator[tuple[int, int], None, None]:
    row, col = point
    row_num = len(heightmap)
    col_num = len(heightmap[0])
    current_height = heightmap[row][col]
    neighbers = get_four_neighbers(row, col, row_num, col_num)
    return (
        (n_row, n_col)
        for n_row, n_col in neighbers
        if ord(heightmap[n_row][n_col]) - ord(current_height) <= 1
    )


def get_neighbers_backword(
    heightmap: list[list[str]], point: tuple[int, int]
) -> Generator[tuple[int, int], None, None]:
    row, col = point
    row_num = len(heightmap)
    col_num = len(heightmap[0])
    current_height = heightmap[row][col]
    neighbers = get_four_neighbers(row, col, row_num, col_num)
    return (
        (n_row, n_col)
        for n_row, n_col in neighbers
        if ord(current_height) - ord(heightmap[n_row][n_col]) <= 1
    )


def part_one(path: str | None = None) -> int:
    with open(path if path else "input/day12_input") as f:
        heightmap, start_point, end_point = parse_heightmap(f)
        return dfs(heightmap, start_point, end_point, get_neighbers_forward)


def part_two(path: str | None = None) -> int:
    with open(path if path else "input/day12_input") as f:
        heightmap, start_point, end_point = parse_heightmap(f)
        return dfs(heightmap, end_point, "a", get_neighbers_backword)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day12_test_input"), 31)

    def test_part_two(self):
        self.assertEqual(part_two("input/day12_test_input"), 29)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
