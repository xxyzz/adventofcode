import unittest

DAY = 22


def parse_input(
    path: str,
) -> tuple[dict[tuple[int, int], str], list[int | str], int, int]:
    with open(path) as f:
        row = 0
        board = {}
        parsing_board = True
        path_str = ""
        start_column = -1
        size = 100
        for line in f:
            if parsing_board:
                if not line.strip():
                    parsing_board = False
                    continue
                if len(line.strip()) < size:
                    size = len(line.strip())
                row += 1
                for column, value in enumerate(line.rstrip(), start=1):
                    if value in [".", "#"]:
                        board[row, column] = value
                        if row == 1 and start_column == -1 and value == ".":
                            start_column = column
            else:
                path_str = line.strip()

        paths: list[int | str] = []
        num = ""
        for c in path_str:
            if c.isdigit():
                num += c
            else:
                paths.append(int(num))
                num = ""
                paths.append(c)
        if num:
            paths.append(int(num))
        return board, paths, start_column, size


DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # right, down, left, up


def part_one(input_path: str) -> int:
    row = 1
    board, paths, column, _ = parse_input(input_path)
    direction = 0
    for path in paths:
        match path:
            case int():
                for _ in range(path):
                    next_row = row + DIRECTIONS[direction][0]
                    next_column = column + DIRECTIONS[direction][1]
                    if (next_row, next_column) not in board:
                        next_row, next_column = wrap_around(
                            row, column, direction, board
                        )
                    if board[next_row, next_column] == "#":
                        break
                    row, column = next_row, next_column
            case "R":
                direction = (direction + 1) % 4
            case "L":
                direction = (direction - 1) % 4

    return 1000 * row + 4 * column + direction


def wrap_around(row, column, direction, board):
    opposite_direction = (direction + 2) % 4
    while True:
        next_row = row + DIRECTIONS[opposite_direction][0]
        next_column = column + DIRECTIONS[opposite_direction][1]
        if (next_row, next_column) not in board:
            return row, column
        row, column = next_row, next_column
    return row, column


def part_two(input_path: str) -> int:
    row = 1
    board, paths, column, size = parse_input(input_path)
    direction = 0
    for path in paths:
        match path:
            case int():
                for _ in range(path):
                    next_row = row + DIRECTIONS[direction][0]
                    next_column = column + DIRECTIONS[direction][1]
                    next_direction = direction
                    if (next_row, next_column) not in board:
                        next_row, next_column, next_direction = wrap_around_cube(
                            row, column, direction, board, size
                        )
                    if board[next_row, next_column] == "#":
                        break
                    row, column, direction = next_row, next_column, next_direction
            case "R":
                direction = (direction + 1) % 4
            case "L":
                direction = (direction - 1) % 4

    return 1000 * row + 4 * column + direction


# https://reddit.com/r/adventofcode/comments/zseg3c/_/j17qdop/
# 0: right, 1: down, 2: left, 3:  up
def wrap_around_cube(row, column, direction, board, size):
    match (row - 1) // size, (column - 1) // size, direction:
        # left edges
        case 0, 1, 3:
            return 3 * size + 1 + (column - 1) % size, 1, 0
        case 3, 0, 2:
            return 1, size + 1 + (row - 1) % size, 1
        case 0, 1, 2:
            return 2 * size + size - (row - 1) % size, 1, 0
        case 2, 0, 2:
            return size - (row - 1) % size, size + 1, 0
        case 1, 1, 2:
            return size * 2 + 1, 1 + (row - 1) % size, 1
        case 2, 0, 3:
            return size + 1 + (column - 1) % size, size + 1, 0
        # right edges
        case 0, 2, 3:
            return size * 4, 1 + (column - 1) % size, 3
        case 3, 0, 1:
            return 1, size * 2 + 1 + (column - 1) % size, 1
        case 0, 2, 0:
            return size * 2 + size - (row - 1) % size, size * 2, 2
        case 2, 1, 0:
            return size - (row - 1) % size, size * 3, 2
        case 0, 2, 1:
            return size + 1 + (column - 1) % size, size * 2, 2
        case 1, 1, 0:
            return size, size * 2 + 1 + (row - 1) % size, 3
        case 2, 1, 1:
            return size * 3 + 1 + (column - 1) % size, size, 2
        case 3, 0, 0:
            return 3 * size, size + 1 + (row - 1) % size, 3


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 6032)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
