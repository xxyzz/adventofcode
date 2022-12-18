import unittest

DAY = 13


def parse_input(input_path: str) -> tuple[set[tuple[int, int]], list[list[str]]]:
    points: set[tuple[int, int]] = set()
    fold_instructions: list[list[str]] = []
    with open(input_path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            if line.startswith("fold"):
                fold_instructions.append(line.split()[-1].split("="))
            else:
                points.add(tuple(map(int, line.split(","))))

    return points, fold_instructions


def fold(points: set[tuple[int, int]], fold_instructions: list[list[str]]) -> int:
    for asix, value_str in fold_instructions:
        value = int(value_str)
        removed_points = set()
        added_points = set()
        if asix == "x":
            for x, y in points:
                if x > value:
                    added_points.add((2 * value - x, y))
                    removed_points.add((x, y))
        elif asix == "y":
            for x, y in points:
                if y > value:
                    added_points.add((x, 2 * value - y))
                    removed_points.add((x, y))
        points -= removed_points
        points |= added_points

    if len(fold_instructions) > 1:
        print_points(points)
    return len(points)


def print_points(points):
    max_x = max(x for x, _ in points)
    max_y = max(y for _, y in points)

    for y in range(max_y + 1):
        for x in range(max_x + 1):
            if (x, y) in points:
                print("#", end="")
            else:
                print(" ", end="")
        print()


def part_one(input_path: str) -> int:
    points, fold_instructions = parse_input(input_path)
    return fold(points, fold_instructions[:1])


def part_two(input_path: str) -> int:
    points, fold_instructions = parse_input(input_path)
    return fold(points, fold_instructions)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 17)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    part_two(f"input/day{DAY}_input")
