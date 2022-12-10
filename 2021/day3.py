import math
import unittest
from collections import Counter


def most_common_bit_at_col(nums: list[int], nth_bit: int) -> int:
    num_of_ones = sum((1 if (num >> nth_bit) & 1 else 0 for num in nums))
    return 1 if num_of_ones >= math.ceil(len(nums) / 2) else 0


def combine_most_common_bit(nums: list[int], num_bits: int) -> int:
    result = 0

    for nth_bit in reversed(range(num_bits)):
        result <<= 1
        result += most_common_bit_at_col(nums, nth_bit)

    return result


def part_one(test_input: str | None = None) -> int:
    with open(test_input if test_input else "input/day3_input") as f:
        lines = [line.strip() for line in f]
        nums = [int(line, 2) for line in lines]
        num_bits = len(lines[0])
        gamma = combine_most_common_bit(nums, num_bits)
        mask = ~(~0 << num_bits)  # n number of 1s
        epsilon = mask ^ gamma  # invert gamma
        return gamma * epsilon


def filter_number(nums: list[int], num_bits: int, prefer_one: bool) -> int:
    for nth_bit in reversed(range(num_bits)):
        choose_bit = most_common_bit_at_col(nums, nth_bit)
        if not prefer_one:
            choose_bit = 1 - choose_bit
        nums = [num for num in nums if (num >> nth_bit) & 1 == choose_bit]

        if len(nums) == 1:
            return nums[0]

    return 0


def part_two(test_input: str | None = None) -> int:
    with open(test_input if test_input else "input/day3_input") as f:
        lines = [line.strip() for line in f]
        nums = [int(line, 2) for line in lines]
        num_bits = len(lines[0])

        oxygen_rating = filter_number(nums, num_bits, True)
        co2_rating = filter_number(nums, num_bits, False)

        return oxygen_rating * co2_rating


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day3_test_input"), 198)

    def test_part_two(self):
        self.assertEqual(part_two("input/day3_test_input"), 230)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
