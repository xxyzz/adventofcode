import unittest


def part_one(input_text: str | None = None) -> int:
    with open(input_text if input_text else "input/day1_input", encoding="utf-8") as f:
        nums = [int(line.strip()) for line in f]
        return sum(a < b for a, b in zip(nums, nums[1:]))


def part_two(input_text: str | None = None) -> int:
    with open(input_text if input_text else "input/day1_input", encoding="utf-8") as f:
        nums = [int(line.strip()) for line in f]
        return sum(a < b for a, b in zip(nums, nums[3:]))


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day1_test_input"), 7)

    def test_part_two(self):
        self.assertEqual(part_two("input/day1_test_input"), 5)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
