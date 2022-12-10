import unittest


def part_one(path: str | None = None) -> int:
    x_value = 1
    cycles = 0
    check_cycle = 20
    signal_strengths = 0
    with open(path if path else "input/day10_input") as f:
        for line in f:
            if line.startswith("noop"):
                cycles += 1
                if cycles == check_cycle:
                    signal_strengths += check_cycle * x_value
                    check_cycle += 40
            else:
                cycles += 1
                if cycles == check_cycle:
                    signal_strengths += check_cycle * x_value
                    check_cycle += 40
                cycles += 1
                if cycles == check_cycle:
                    signal_strengths += check_cycle * x_value
                    check_cycle += 40
                x_value += int(line.split()[-1])

    return signal_strengths


def part_two(path: str | None = None):
    x_value = 1
    cycles = 0
    with open(path if path else "input/day10_input") as f:
        for line in f:
            if line.startswith("noop"):
                cycles += 1
                print_screen(x_value, cycles)
            else:
                cycles += 1
                print_screen(x_value, cycles)
                cycles += 1
                print_screen(x_value, cycles)
                x_value += int(line.split()[-1])


def print_screen(x_value, cycles):
    print_position = (cycles - 1) % 40
    if print_position in {x_value - 1, x_value, x_value + 1}:
        print("#", end="")
    else:
        print(" ", end="")  # easier to read
    if cycles % 40 == 0:
        print()


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day10_test_input"), 13140)

    def test_part_two(self):
        part_two("input/day10_test_input")
        self.assertEqual(1, 1)


if __name__ == "__main__":
    print(part_one())
    part_two()
