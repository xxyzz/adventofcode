import unittest


def part_one(test_input: str | None = None) -> int:
    with open(test_input if test_input else "day2_input") as f:
        x_value = 0
        depth = 0
        for line in f:
            command, value_str = line.strip().split()
            value = int(value_str)
            match command:
                case "forward":
                    x_value += value
                case "down":
                    depth += value
                case "up":
                    depth -= value

        return x_value * depth


def part_two(test_input: str | None = None) -> int:
    with open(test_input if test_input else "day2_input") as f:
        x_value = 0
        depth = 0
        aim = 0
        for line in f:
            command, value_str = line.strip().split()
            value = int(value_str)
            match command:
                case "forward":
                    x_value += value
                    depth += aim * value
                case "down":
                    aim += value
                case "up":
                    aim -= value

        return x_value * depth


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("day2_test_input"), 150)

    def test_part_two(self):
        self.assertEqual(part_two("day2_test_input"), 900)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
