import unittest

DAY = 21


def parse_monkeyes(path: str) -> dict[str, int | list[str]]:
    monkeyes: dict[str, int | list[str]] = {}
    with open(path) as f:
        for line in f:
            monkey, value = line.split(":")
            if value.strip().isdigit():
                monkeyes[monkey] = int(value.strip())
            else:
                monkeyes[monkey] = value.split()
    return monkeyes


def find_monkey_value(monkey: str, monkeyes: dict[str, int | list[str]]) -> int:
    value = monkeyes[monkey]
    if value is None:
        return None
    if type(value) == int:
        return value
    monkey_a, operation, monkey_b = value
    monkey_a_value = find_monkey_value(monkey_a, monkeyes)
    monkey_b_value = find_monkey_value(monkey_b, monkeyes)
    if monkey_a_value is None or monkey_b_value is None:
        return None
    match operation:
        case "+":
            return monkey_a_value + monkey_b_value
        case "-":
            return monkey_a_value - monkey_b_value
        case "*":
            return monkey_a_value * monkey_b_value
        case "/":
            return monkey_a_value // monkey_b_value


def part_one(path: str) -> int:
    return find_monkey_value("root", parse_monkeyes(path))


# https://reddit.com/r/adventofcode/comments/zrav4h/2022_day_21_solutions/j12ro93
def part_two(path: str) -> int:
    monkeyes = parse_monkeyes(path)
    monkeyes["humn"] = None
    monkey_a, _, monkey_b = monkeyes["root"]
    monkey_a_val = find_monkey_value(monkey_a, monkeyes)
    monkey_b_val = find_monkey_value(monkey_b, monkeyes)
    value = monkey_b_val if monkey_a_val is None else monkey_a_val
    monkey = monkey_a if monkey_a_val is None else monkey_b
    while True:
        if monkey == "humn":
            return value
        monkey_a, operation, monkey_b = monkeyes[monkey]
        monkey_a_val = find_monkey_value(monkey_a, monkeyes)
        monkey_b_val = find_monkey_value(monkey_b, monkeyes)
        monkey = monkey_a if monkey_a_val is None else monkey_b
        match monkey_a_val, operation, monkey_b_val:
            case int, "+", None:
                value = value - monkey_a_val
            case None, "+", int:
                value = value - monkey_b_val
            case int, "-", None:
                value = monkey_a_val - value
            case None, "-", int:
                value = value + monkey_b_val
            case int, "*", None:
                value = value // monkey_a_val
            case None, "*", int:
                value = value // monkey_b_val
            case int, "/", None:
                value = monkey_a_val // value
            case None, "/", int:
                value = value * monkey_b_val
    return -1


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 152)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 301)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
