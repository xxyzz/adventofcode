import itertools
import unittest

DAY = 25


SeaFloorMap = set[tuple[int, int]]


def parse_input(path: str) -> tuple[SeaFloorMap, SeaFloorMap, int, int]:
    with open(path) as f:
        east_facing: SeaFloorMap = set()
        south_facing: SeaFloorMap = set()
        for row, line in enumerate(f):
            for col, direction in enumerate(line.strip()):
                if direction == ">":
                    east_facing.add((row, col))
                elif direction == "v":
                    south_facing.add((row, col))
    return east_facing, south_facing, row + 1, col + 1


def move(
    dr: int,
    dc: int,
    moved_map: SeaFloorMap,
    other_map: SeaFloorMap,
    row_count: int,
    col_count: int,
) -> tuple[bool, SeaFloorMap]:
    new_moved_map = set()
    moved = False
    for row, col in moved_map:
        new_row = (row + dr) % row_count
        new_col = (col + dc) % col_count
        new_pos = (new_row, new_col)
        if new_pos not in moved_map and new_pos not in other_map:
            moved = True
            new_moved_map.add(new_pos)
        else:
            new_moved_map.add((row, col))
    return moved, new_moved_map


def part_one(input_path: str) -> int:
    east_facing, south_facing, row_count, col_count = parse_input(input_path)
    for steps in itertools.count(1):
        moved_east, new_east = move(
            0, 1, east_facing, south_facing, row_count, col_count
        )
        east_facing = new_east
        moved_south, new_south = move(
            1, 0, south_facing, east_facing, row_count, col_count
        )
        south_facing = new_south
        if not moved_east and not moved_south:
            return steps
    return 0


def print_map(east: SeaFloorMap, south: SeaFloorMap, row_count, col_count):
    for row in range(row_count):
        for col in range(col_count):
            pos = (row, col)
            if pos in east:
                print(">", end="")
            elif pos in south:
                print("v", end="")
            else:
                print(".", end="")
        print()


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 58)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
