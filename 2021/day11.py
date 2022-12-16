import unittest

DAY = 11


def adjacent_octopuses(octopuses, row, col):
    row_num = len(octopuses)
    col_num = len(octopuses[0])
    for dr, dc in (
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ):
        nr = row + dr
        nc = col + dc
        if 0 <= nr < row_num and 0 <= nc < col_num:
            yield (nr, nc)


def flash_adjacent_octopuses(octopuses, row, col, flashed):
    for nr, nc in adjacent_octopuses(octopuses, row, col):
        if (nr, nc) not in flashed:
            octopuses[nr][nc] += 1
            if octopuses[nr][nc] > 9:
                flashed.add((nr, nc))
                octopuses[nr][nc] = 0
                flash_adjacent_octopuses(octopuses, nr, nc, flashed)


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        octopuses = [list(map(int, line.strip())) for line in f]
        flashes = 0
        row_num = len(octopuses)
        col_num = len(octopuses[0])
        for _ in range(100):
            flashed = set()
            for row in range(row_num):
                for col in range(col_num):
                    if (row, col) not in flashed:
                        octopuses[row][col] += 1
                        if octopuses[row][col] > 9:
                            flashed.add((row, col))
                            octopuses[row][col] = 0
                            flash_adjacent_octopuses(octopuses, row, col, flashed)
            flashes += len(flashed)
        return flashes


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        octopuses = [list(map(int, line.strip())) for line in f]
        row_num = len(octopuses)
        col_num = len(octopuses[0])
        steps = 0
        while True:
            steps += 1
            flashed = set()
            for row in range(row_num):
                for col in range(col_num):
                    if (row, col) not in flashed:
                        octopuses[row][col] += 1
                        if octopuses[row][col] > 9:
                            flashed.add((row, col))
                            octopuses[row][col] = 0
                            flash_adjacent_octopuses(octopuses, row, col, flashed)
            if sum(sum(row) for row in octopuses) == 0:
                break
        return steps


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 1656)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 195)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
