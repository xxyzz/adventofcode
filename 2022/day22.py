import unittest

DAY = 22


def parse_input(path: str) -> tuple[dict[tuple[int, int], str], list[int | str], int]:
    with open(path) as f:
        row = 0
        board = {}
        parsing_board = True
        path_str = ""
        start_column = -1
        for line in f:
            if parsing_board:
                if not line.strip():
                    parsing_board = False
                    continue
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
        return board, paths, start_column


DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # right, down, left, up


def part_one(input_path: str) -> int:
    row = 1
    board, paths, column = parse_input(input_path)
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

    # print(f"{row=} {column=} {direction=}")
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


def part_two(path: str) -> int:
    pass


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 6032)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
