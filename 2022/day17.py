import unittest

DAY = 17


def init_rock_position(x: int, y: int, rock_type: int) -> list[tuple[int, int]]:
    match rock_type:
        case 0:  # -
            return [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
        case 1:  # +
            return [
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y + 2),
            ]
        case 2:  # L
            return [(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]
        case 3:  # |
            return [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
        case 4:  # cube
            return [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
        case _:
            return [(0, 0)]


def part_one(path: str, loop: int = 2022) -> int:
    with open(path) as f:
        jet_patterns = f.read().strip()
        jet_len = len(jet_patterns)
        highest_y = -1
        grid: set[tuple[int, int]] = set()
        j = 0
        height_pattern: dict[tuple[int, int], tuple[int, int]] = {}
        for i in range(loop):
            new_rock = init_rock_position(2, highest_y + 4, i % 5)
            # print("new rock")
            # print_grid(grid | set(new_rock))
            while True:
                jet = jet_patterns[j % jet_len]
                j += 1

                if jet == "<":
                    move_left(new_rock, grid)
                else:
                    move_right(new_rock, grid)

                # print(jet)
                # print_grid(grid | set(new_rock))

                if move_down(new_rock, grid):
                    rock_highest_y = max(y for x, y in new_rock)
                    highest_y = max(rock_highest_y, highest_y)
                    # print("down")
                    # print_grid(grid | set(new_rock))
                    break

            if (i % 5, j % jet_len) in height_pattern:
                prev_loop, prev_height = height_pattern[i % 5, j % jet_len]
                quotient, remainder = divmod(loop - i, i - prev_loop)
                if remainder == 0:
                    result = quotient * (highest_y - prev_height) + highest_y
                    return result if loop % 10 == 0 else result - 1  # weird off-by-one
            else:
                height_pattern[i % 5, j % jet_len] = i, highest_y

        return highest_y + 1


def print_grid(grid: set[tuple[int, int]]) -> None:
    for y in reversed(range(10)):
        for x in range(7):
            if (x, y) in grid:
                print("#", end="")
            else:
                print(".", end="")
        print()
    print()


def move_left(rock: list[tuple[int, int]], grid: set[tuple[int, int]]) -> None:
    if any(x == 0 or (x - 1, y) in grid for x, y in rock):
        return
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x - 1, y)


def move_right(rock: list[tuple[int, int]], grid: set[tuple[int, int]]) -> None:
    if any(x == 6 or (x + 1, y) in grid for x, y in rock):
        return
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x + 1, y)


def move_down(rock: list[tuple[int, int]], grid: set[tuple[int, int]]) -> bool:
    if any(y == 0 or (x, y - 1) in grid for x, y in rock):
        for point in rock:
            grid.add(point)
        return True  # rest
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x, y - 1)
    return False


def part_two(path: str) -> int:
    return part_one(path, 1_000_000_000_000)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 3068)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 1_514_285_714_288)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
