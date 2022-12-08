import unittest


def part_one(input_text: str | None = None) -> int:
    with open(input_text if input_text else "day1_input", encoding="utf-8") as f:
        result = 0
        values = [int(line) for line in f]
        for i in range(1, len(values)):
            if values[i] > values[i - 1]:
                result += 1
        return result


def part_two(input_text: str | None = None) -> int:
    with open(input_text if input_text else "day1_input", encoding="utf-8") as f:
        result = 0
        values = [int(line) for line in f]
        for i in range(len(values) - 3):
            first_window = values[i : i + 3]
            second_window = values[i + 1 : i + 4]
            if sum(second_window) > sum(first_window):
                result += 1
        return result


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEquals(part_one("day1_test_input"), 7)

    def test_part_two(self):
        self.assertEquals(part_two("day1_test_input"), 5)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
