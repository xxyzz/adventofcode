import unittest

DAY = 17


def rock_one(x, y):  # -
    return [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]


def rock_two(x, y):  # +
    return [(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)]


def rock_three(x, y):  # L
    return [(x, y), (x + 1, y), (x + 2, y), (x + 2, y + 1), (x + 2, y + 2)]


def rock_four(x, y):  # |
    return [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]


def rock_five(x, y):  # cube
    return [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]


def part_one(path: str, loop: int = 2022) -> int:
    with open(path) as f:
        jet_patterns = f.read().strip()
        jet_len = len(jet_patterns)
        rocks = [rock_one, rock_two, rock_three, rock_four, rock_five]
        highest_y = -1
        grid = set()
        j = 0
        for i in range(loop):
            new_rock = rocks[i % 5](2, highest_y + 4)
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
                    print(f"{i=} {highest_y}")
                    # print("down")
                    # print_grid(grid | set(new_rock))
                    break
        return highest_y + 1


def print_grid(grid):
    for y in reversed(range(10)):
        for x in range(7):
            if (x, y) in grid:
                print("#", end="")
            else:
                print(".", end="")
        print()
    print()


def move_left(rock, grid):
    if any(x == 0 or (x - 1, y) in grid for x, y in rock):
        return
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x - 1, y)


def move_right(rock, grid):
    if any(x == 6 or (x + 1, y) in grid for x, y in rock):
        return
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x + 1, y)


def move_down(rock, grid):
    if any(y == 0 or (x, y - 1) in grid for x, y in rock):
        for point in rock:
            grid.add(point)
        return True  # rest
    for i in range(len(rock)):
        x, y = rock[i]
        rock[i] = (x, y - 1)


def part_two(path: str) -> int:
    return part_one(path, 1000000000000)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 3068)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), 1514285714288)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
