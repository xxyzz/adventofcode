import math
import unittest

DAY = 20

# algo_str[0] is "#", all dark pixels are turned to light pixels
# but algo_str[255] is ".", these pixels will be dark in next round
def parse_input(path: str) -> tuple[str, set[tuple[int, int]]]:
    algo_str = ""
    input_img = set()
    parsing_img = False
    row = 0
    with open(path) as f:
        for line in f:
            if not line.strip():
                parsing_img = True
                continue
            if not parsing_img:
                algo_str = line.strip()
            else:
                for col, pixel in enumerate(line.strip()):
                    if pixel == "#":
                        input_img.add((row, col))
                row += 1
    return algo_str, input_img


def combine_sorrounding_pixel(
    position: tuple[int, int], input_img: set[tuple[int, int]], light_pixel_map: bool
) -> int:
    row, col = position
    number = 0
    for dr in [-1, 0, 1]:
        for dc in [-1, 0, 1]:
            new_row = row + dr
            new_col = col + dc
            number <<= 1
            if light_pixel_map:
                if (new_row, new_col) in input_img:
                    number += 1
            elif (new_row, new_col) not in input_img:
                number += 1
    return number


def enhance(
    input_img: set[tuple[int, int]], algo_str: str, light_pixel_map: bool
) -> set[tuple[int, int]]:
    new_img = set()
    min_row = math.inf
    max_row = -math.inf
    min_col = math.inf
    max_col = -math.inf
    for row, col in input_img:
        min_row = min(row, min_row)
        max_row = max(row, max_row)
        min_col = min(col, min_col)
        max_col = max(col, max_col)

    min_row -= 2
    max_row += 2
    min_col -= 2
    max_col += 2

    check_pixel = "." if light_pixel_map else "#"

    for row in range(min_row, max_row + 1):
        for col in range(min_col, max_col + 1):
            index = combine_sorrounding_pixel((row, col), input_img, light_pixel_map)
            if algo_str[index] == check_pixel:
                new_img.add((row, col))

    # print_img(new_img, min_row, max_row, min_col, max_col)
    return new_img


def print_img(img, min_r, max_r, min_c, max_c):
    for r in range(min_r, max_r + 1):
        for c in range(min_c, max_c + 1):
            if (r, c) in img:
                print("#", end="")
            else:
                print(".", end="")
        print()
    print()


def part_one(input_path: str) -> int:
    algo_str, input_img = parse_input(input_path)
    for round_num in range(2):
        input_img = enhance(input_img, algo_str, round_num % 2 == 0)
    return len(input_img)


def part_two(input_path: str) -> int:
    algo_str, input_img = parse_input(input_path)
    for round_num in range(50):
        input_img = enhance(input_img, algo_str, round_num % 2 == 0)
    return len(input_img)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 35)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 3351)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
