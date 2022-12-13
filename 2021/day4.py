import unittest


def part_one(test_input: str | None = None) -> int:
    boards: list[list[list[int]]] = []
    board_nums: list[set[int]] = []
    nums = []
    with open(test_input if test_input else "input/day4_input") as f:
        line_chunks = f.read().split("\n\n")

        for line_chunk in line_chunks:
            if "," in line_chunk:
                nums = [int(n) for n in line_chunk.strip().split(",")]
            else:
                boards.append(
                    [list(map(int, row.split())) for row in line_chunk.splitlines()]
                )
                board_nums.append(
                    {boards[-1][row][col] for row in range(5) for col in range(5)}
                )

        for num in nums:
            for board, board_num in zip(boards, board_nums):
                r = mark_boards(board, board_num, num)
                if r > 0:
                    return r

        return -1


def mark_boards(board, board_num, num):
    if num in board_num:
        for row in range(5):
            for col in range(5):
                if board[row][col] == num:
                    board[row][col] = -1
                    board_num.remove(num)

                if check_win(board, row, col):
                    result = num * sum(board_num)
                    board_num.clear()
                    return result

    return -1


def check_win(board: list[list[int]], row: int, col: int) -> bool:
    if all(board[row][c] == -1 for c in range(5)):
        return True

    if all(board[r][col] == -1 for r in range(5)):
        return True

    return False


def part_two(test_input: str | None = None) -> int:
    boards: list[list[list[int]]] = []
    board_nums: list[set[int]] = []
    nums = []
    with open(test_input if test_input else "input/day4_input") as f:
        line_chunks = f.read().split("\n\n")

        for line_chunk in line_chunks:
            if "," in line_chunk:
                nums = [int(n) for n in line_chunk.strip().split(",")]
            else:
                boards.append(
                    [list(map(int, row.split())) for row in line_chunk.splitlines()]
                )
                board_nums.append(
                    {boards[-1][row][col] for row in range(5) for col in range(5)}
                )

        num_of_boards = len(boards)
        win_nums = 0

        for num in nums:
            for board, board_num in zip(boards, board_nums):
                r = mark_boards(board, board_num, num)
                if r > 0:
                    win_nums += 1
                    if win_nums >= num_of_boards:
                        return r

        return -1


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day4_test_input"), 4512)

    def test_part_two(self):
        self.assertEqual(part_two("input/day4_test_input"), 1924)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
