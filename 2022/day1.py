import unittest


def part_one(input_file: str | None = None) -> int:
    with open(input_file if input_file else "input/day1_input") as f:
        elf_calories = f.read().split("\n\n")
        return max(
            sum(int(calorie) for calorie in calories.split())
            for calories in elf_calories
        )


def part_two(input_file: str | None = None) -> int:
    with open(input_file if input_file else "input/day1_input") as f:
        elf_calories = f.read().split("\n\n")
        sum_elf_calories = [
            sum(int(calorie) for calorie in calories.split())
            for calories in elf_calories
        ]
        sum_elf_calories.sort(reverse=True)
        return sum(sum_elf_calories[:3])


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day1_test_input"), 24000)

    def test_part_two(self):
        self.assertEqual(part_two("input/day1_test_input"), 45000)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
