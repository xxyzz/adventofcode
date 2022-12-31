import unittest
from collections import defaultdict, deque

DAY = 24


def parse_map(
    path: str,
) -> tuple[dict[tuple[int, int], str], int, int, tuple[int, int]]:
    with open(path) as f:
        blizzard_map = {}
        end = None
        row_num = 0
        col_num = 0
        for row, row_str in enumerate(f):
            row_num += 1
            col_num = len(row_str.strip())
            for col, value in enumerate(row_str.strip()):
                if value == "#":
                    blizzard_map[row, col] = "#"
                elif value != ".":
                    blizzard_map[row, col] = value
        end = row_num - 1, col_num - 2
        return blizzard_map, row_num, col_num, end


def compute_new_blizzard_map(old_map, row_num, col_num):
    new_blizzard_map = defaultdict(str)
    for blizzard_loc, blizzard_directions in old_map.items():
        if blizzard_directions == "#":
            new_blizzard_map[blizzard_loc] = "#"
        else:
            for blizzard_direction in blizzard_directions:
                blizzard_row, blizzard_col = blizzard_loc
                match blizzard_direction:
                    case "^":
                        new_blizzard_loc = blizzard_row - 1, blizzard_col
                    case "v":
                        new_blizzard_loc = blizzard_row + 1, blizzard_col
                    case "<":
                        new_blizzard_loc = blizzard_row, blizzard_col - 1
                    case ">":
                        new_blizzard_loc = blizzard_row, blizzard_col + 1
                if old_map.get(new_blizzard_loc) == "#":
                    match blizzard_direction:
                        case "^":
                            new_blizzard_loc = row_num - 2, blizzard_col
                        case "v":
                            new_blizzard_loc = 1, blizzard_col
                        case "<":
                            new_blizzard_loc = blizzard_row, col_num - 2
                        case ">":
                            new_blizzard_loc = blizzard_row, 1
                new_blizzard_map[new_blizzard_loc] += blizzard_direction
    return new_blizzard_map


def part_one(input_path: str) -> int:
    init_blizzard_map, row_num, col_num, end = parse_map(input_path)
    blizzard_maps = {0: init_blizzard_map}
    q: deque[tuple[tuple[int, int], int]] = deque()
    q.append(((0, 1), 0))
    visited = set()
    while q:
        my_position, steps = q.popleft()
        if (my_position, steps) in visited:
            continue
        visited.add((my_position, steps))
        if steps + 1 in blizzard_maps:
            blizzard_map = blizzard_maps[steps + 1]
        else:
            blizzard_map = compute_new_blizzard_map(
                blizzard_maps[steps], row_num, col_num
            )
            blizzard_maps[steps + 1] = blizzard_map

        row, col = my_position
        for dr, dc in [(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)]:
            new_row = row + dr
            new_col = col + dc
            if (
                (new_row, new_col) not in blizzard_map
                and 0 <= new_row < row_num
                and 0 <= new_col < col_num
            ):
                if (new_row, new_col) == end:
                    return steps + 1
                q.append(((new_row, new_col), steps + 1))

    return 0


def test_blizzard(path):
    blizzard_map, row_num, col_num, end = parse_map(path)
    print(f"{row_num=} {col_num=} {end=}")
    print_map(blizzard_map, row_num, col_num)
    for _ in range(5):
        new_blizzard_map = defaultdict(str)
        for blizzard_loc, blizzard_directions in blizzard_map.items():
            if blizzard_directions == "#":
                new_blizzard_map[blizzard_loc] = "#"
            else:
                for blizzard_direction in blizzard_directions:
                    blizzard_row, blizzard_col = blizzard_loc
                    match blizzard_direction:
                        case "^":
                            new_blizzard_loc = blizzard_row - 1, blizzard_col
                        case "v":
                            new_blizzard_loc = blizzard_row + 1, blizzard_col
                        case "<":
                            new_blizzard_loc = blizzard_row, blizzard_col - 1
                        case ">":
                            new_blizzard_loc = blizzard_row, blizzard_col + 1
                    if blizzard_map.get(new_blizzard_loc) == "#":
                        match blizzard_direction:
                            case "^":
                                new_blizzard_loc = row_num - 2, blizzard_col
                            case "v":
                                new_blizzard_loc = 1, blizzard_col
                            case "<":
                                new_blizzard_loc = blizzard_row, col_num - 2
                            case ">":
                                new_blizzard_loc = blizzard_row, 1
                    new_blizzard_map[new_blizzard_loc] += blizzard_direction
        blizzard_map = new_blizzard_map
        print_map(blizzard_map, row_num, col_num)


def print_map(blizzard_map, row_num, col_num):
    for row in range(row_num):
        for col in range(col_num):
            value = blizzard_map.get((row, col))
            if value is None:
                print(".", end="")
            elif value == "#":
                print("#", end="")
            elif len(value) > 1:
                print(len(value), end="")
            elif len(value) == 1:
                print(value, end="")
        print()
    print()


def part_two(path: str) -> int:
    pass


class Test(unittest.TestCase):
    def test_part_one(self):
        # test_blizzard(f"input/day{DAY}_test_input1")
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 18)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
