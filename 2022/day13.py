import unittest
from functools import cmp_to_key
from itertools import zip_longest


def part_one(path: str) -> int:
    with open(path) as f:
        packet_pairs = []
        result = 0
        for pair_index, pairs in enumerate(f.read().split("\n\n")):
            p_a, p_b = pairs.strip().splitlines()
            p_a = eval(p_a)
            p_b = eval(p_b)
            if check_order(p_a, p_b):
                result += pair_index + 1
        return result


def check_order(p_a, p_b):
    if isinstance(p_a, int) and isinstance(p_b, int):
        if p_a > p_b:
            return False
        elif p_a < p_b:
            return True
    if isinstance(p_a, list) and isinstance(p_b, list):
        for e_a, e_b in zip_longest(p_a, p_b):
            if e_a is None and e_b is not None:
                return True
            if e_a is not None and e_b is None:
                return False
            r = check_order(e_a, e_b)
            if r is not None:
                return r
    if isinstance(p_a, int) and isinstance(p_b, list):
        return check_order([p_a], p_b)
    if isinstance(p_a, list) and isinstance(p_b, int):
        return check_order(p_a, [p_b])


def compare(a, b):
    r = check_order(a, b)
    match r:
        case None:
            return 0
        case True:
            return -1
        case False:
            return 1


def part_two(path: str) -> int:
    with open(path) as f:
        packets = [[[2]], [[6]]]
        result = 0
        for pair_index, pairs in enumerate(f.read().split("\n\n")):
            p_a, p_b = pairs.strip().splitlines()
            packets.append(eval(p_a))
            packets.append(eval(p_b))

        packets.sort(key=cmp_to_key(compare))
        result = 1
        for i, p in enumerate(packets):
            if p == [[2]] or p == [[6]]:
                result *= i + 1
        return result


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day13_test_input"), 13)

    def test_part_two(self):
        self.assertEqual(part_two("input/day13_test_input"), 140)


if __name__ == "__main__":
    print(part_one("input/day13_input"))
    print(part_two("input/day13_input"))
