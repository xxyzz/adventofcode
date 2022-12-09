import unittest


def part_one(path: str | None = None) -> int:
    visited_spots = {(0, 0)}
    head = [0, 0]
    tail = [0, 0]
    with open(path if path else "src/day9_input") as f:
        for line in f:
            direction, steps_str = line.strip().split()
            steps = int(steps_str)

            for _ in range(steps):
                match direction:
                    case "R":
                        head[0] += 1
                    case "L":
                        head[0] -= 1
                    case "U":
                        head[1] += 1
                    case "D":
                        head[1] -= 1

                move(head, tail)
                visited_spots.add(tuple(tail))

    return len(visited_spots)


def part_two(path: str | None = None) -> int:
    visited_spots = {(0, 0)}
    head = [0, 0]
    tails = [[0, 0] for _ in range(9)]
    with open(path if path else "src/day9_input") as f:
        for line in f:
            direction, steps_str = line.strip().split()
            steps = int(steps_str)

            for _ in range(steps):
                match direction:
                    case "R":
                        head[0] += 1
                    case "L":
                        head[0] -= 1
                    case "U":
                        head[1] += 1
                    case "D":
                        head[1] -= 1

                move(head, tails[0])
                for i in range(1, 9):
                    move(tails[i - 1], tails[i])
                visited_spots.add(tuple(tails[-1]))

    return len(visited_spots)


def move(head, tail):
    if head != tail and head not in near_spots(tail):
        if head[0] == tail[0]:
            if head[1] > tail[1]:
                tail[1] += 1
            else:
                tail[1] -= 1
        elif head[1] == tail[1]:
            if head[0] > tail[0]:
                tail[0] += 1
            else:
                tail[0] -= 1
        elif head not in diagonal_spots(tail):
            if head[1] > tail[1]:
                tail[1] += 1
            else:
                tail[1] -= 1

            if head[0] > tail[0]:
                tail[0] += 1
            else:
                tail[0] -= 1


def diagonal_spots(loc: list[int]) -> list[list[int]]:
    x, y = loc
    return [[x - 1, y + 1], [x + 1, y + 1], [x - 1, y - 1], [x + 1, y - 1]]


def near_spots(loc: list[int]) -> list[list[int]]:
    x, y = loc
    return [[x - 1, y], [x + 1, y], [x, y + 1], [x, y - 1]]


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("src/day9_test_input"), 13)

    def test_part_two(self):
        self.assertEqual(part_two("src/day9_test_input"), 1)

    def test_part_two_two(self):
        self.assertEqual(part_two("src/day9_test_two_input"), 36)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
