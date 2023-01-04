import functools
import itertools
import json
import math
import unittest
from typing import Union

# https://github.com/benediktwerner/AdventOfCode/blob/master/2021/day18/sol.py
DAY = 18

SnailfishNum = Union[int, list["SnailfishNum"]]


def add_num_from_left(num: SnailfishNum, left_num: SnailfishNum | None) -> SnailfishNum:
    if left_num is None:
        return num
    if type(num) == int:
        return num + left_num
    left_pair, right_pair = num
    return [add_num_from_left(left_pair, left_num), right_pair]


def add_num_from_right(
    num: SnailfishNum, right_num: SnailfishNum | None
) -> SnailfishNum:
    if right_num is None:
        return num
    if type(num) == int:
        return num + right_num
    left_pair, right_pair = num
    return [left_pair, add_num_from_right(right_pair, right_num)]


def explode_snailfish_num(
    num: SnailfishNum, depth: int
) -> tuple[bool, SnailfishNum | None, SnailfishNum, SnailfishNum | None]:
    if type(num) == int:
        return False, None, num, None
    left_pair, right_pair = num
    if depth == 4:
        return True, left_pair, 0, right_pair
    (
        left_pair_exploded,
        exploded_left,
        new_left_pair,
        exploded_right,
    ) = explode_snailfish_num(left_pair, depth + 1)
    if left_pair_exploded:
        return (
            True,
            exploded_left,
            [new_left_pair, add_num_from_left(right_pair, exploded_right)],
            None,
        )
    (
        right_pair_exploded,
        exploded_left,
        new_right_pair,
        exploded_right,
    ) = explode_snailfish_num(right_pair, depth + 1)
    if right_pair_exploded:
        return (
            True,
            None,
            [add_num_from_right(left_pair, exploded_left), new_right_pair],
            exploded_right,
        )
    return False, None, num, None


def split_snailfish_num(num: SnailfishNum) -> tuple[bool, SnailfishNum]:
    match num:
        case int():
            if num >= 10:
                return True, [num // 2, math.ceil(num / 2)]
            else:
                return False, num
        case _:
            left_pair, right_pair = num
            left_pair_splited, new_left_pair = split_snailfish_num(left_pair)
            if left_pair_splited:
                return True, [new_left_pair, right_pair]
            right_pair_splited, new_right_pair = split_snailfish_num(right_pair)
            return right_pair_splited, [left_pair, new_right_pair]


def add_snailfish_num(num_a: SnailfishNum, num_b: SnailfishNum) -> SnailfishNum:
    new_num = [num_a, num_b]
    while True:
        exploded, _, new_num, _ = explode_snailfish_num(new_num, 0)
        if exploded:
            continue
        splited, new_num = split_snailfish_num(new_num)
        if not splited:
            break
    return new_num


def magnitude(num: SnailfishNum) -> int:
    match num:
        case int():
            return num
        case _:
            return 3 * magnitude(num[0]) + 2 * magnitude(num[1])


def parse_snailfish_nums(input_path: str) -> list[SnailfishNum]:
    with open(input_path) as f:
        return [json.loads(line.strip()) for line in f]


def part_one(input_path: str) -> int:
    return magnitude(
        functools.reduce(add_snailfish_num, parse_snailfish_nums(input_path))
    )


def part_two(input_path: str) -> int:
    return max(
        magnitude(add_snailfish_num(num_a, num_b))
        for num_a, num_b in itertools.permutations(parse_snailfish_nums(input_path), 2)
    )


class Test(unittest.TestCase):
    def test_explode(self):
        self.assertEqual(
            explode_snailfish_num([[[[[9, 8], 1], 2], 3], 4], 0)[2],
            [[[[0, 9], 2], 3], 4],
        )
        self.assertEqual(
            explode_snailfish_num([7, [6, [5, [4, [3, 2]]]]], 0)[2],
            [7, [6, [5, [7, 0]]]],
        )
        self.assertEqual(
            explode_snailfish_num([[6, [5, [4, [3, 2]]]], 1], 0)[2],
            [[6, [5, [7, 0]]], 3],
        )
        self.assertEqual(
            explode_snailfish_num([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]], 0)[2],
            [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]],
        )
        self.assertEqual(
            explode_snailfish_num([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]], 0)[2],
            [[3, [2, [8, 0]]], [9, [5, [7, 0]]]],
        )

    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input1"), 3488)
        self.assertEqual(part_one(f"input/day{DAY}_test_input2"), 4140)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input2"), 3993)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
