import unittest


def part_one(path: str) -> int:
    with open(path) as f:
        packet_pairs = [[*map(eval, pair.split())] for pair in f.read().split("\n\n")]
        return sum(i for i, pair in enumerate(packet_pairs, 1) if less_than(*pair))


def less_than(left , right):
    match left, right:
        case int(), int():
            if left < right:
                return True
            elif left > right:
                return False
        case list(), int():
            return less_than(left, [right])
        case int(), list():
            return less_than([left], right)
        case list(), list():
            for r in map(less_than, left, right):
                if r is not None:
                    return r
            return less_than(len(left), len(right))


def part_two(path: str) -> int:
    with open(path) as f:
        packets = [eval(packet) for packet in f.read().split() if packet]
        less_than_two = 1
        less_than_six = 2
        for packet in packets:
            if less_than(packet, [[2]]):
                less_than_two += 1
            if less_than(packet, [[6]]):
                less_than_six += 1
        return less_than_two * less_than_six


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day13_test_input"), 13)

    def test_part_two(self):
        self.assertEqual(part_two("input/day13_test_input"), 140)


if __name__ == "__main__":
    print(part_one("input/day13_input"))
    print(part_two("input/day13_input"))
