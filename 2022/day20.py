import unittest
import re
import functools

DAY = 20


def part_one(path: str) -> int:
    with open(path) as f:
        nums = list(int(line.strip()) for line in f)
        nums_len = len(nums)
        indices = list(range(nums_len))
        for i in indices.copy():
            j = indices.index(i)
            indices.pop(j)
            indices.insert((j + nums[i]) % (nums_len - 1), i)

        zero_index = indices.index(nums.index(0))
        return sum(nums[indices[(zero_index + x) % nums_len]] for x in [1000, 2000, 3000])


def part_two(path: str) -> int:
    with open(path) as f:
        nums = list(int(line.strip()) * 811589153 for line in f)
        nums_len = len(nums)
        indices = list(range(nums_len))
        for i in indices * 10:
            j = indices.index(i)
            indices.pop(j)
            indices.insert((j + nums[i]) % (nums_len - 1), i)

        zero_index = indices.index(nums.index(0))
        return sum(nums[indices[(zero_index + x) % nums_len]] for x in [1000, 2000, 3000])


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 3)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 1623178306)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
