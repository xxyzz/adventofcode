import functools
import math
import operator
import unittest
from typing import Callable


class Monkey:
    items: list[int] = []
    operation: Callable[[int, int], int] = operator.add
    operation_val: str | int = 0
    test_divisible: int = 0
    true_throw: int = 0
    false_throw: int = 0
    processed_items: int = 0


def parse_monkeys(text: str) -> list[Monkey]:
    monkeys = []
    for line_chunk in text.split("\n\n"):
        monkey = Monkey()
        _, items_str, op_str, test_str, true_str, false_str = line_chunk.splitlines()
        monkey.items = list(map(int, items_str.strip().split(":")[-1].split(",")))
        ops = op_str[len("  Operation: new = old ") :].split()
        match ops[0]:
            case "+":
                monkey.operation = operator.add
            case "-":
                monkey.operation = operator.sub
            case "*":
                monkey.operation = operator.mul
            case "/":
                monkey.operation = operator.floordiv

        if ops[1] == "old":
            monkey.operation_val = "old"
        else:
            monkey.operation_val = int(ops[1])
        monkey.test_divisible = int(test_str.split()[-1])
        monkey.true_throw = int(true_str.split()[-1])
        monkey.false_throw = int(false_str.split()[-1])
        monkeys.append(monkey)

    return monkeys


def run(monkeys: list[Monkey], loops: int) -> int:
    lcm = math.lcm(*[monkey.test_divisible for monkey in monkeys])

    for _ in range(loops):
        for monkey in monkeys:
            for item_worry_level in monkey.items.copy():
                monkey.items.remove(item_worry_level)
                if isinstance(monkey.operation_val, str):
                    item_worry_level = monkey.operation(
                        item_worry_level, item_worry_level
                    )
                else:
                    item_worry_level = monkey.operation(
                        item_worry_level, monkey.operation_val
                    )
                if loops == 20:
                    item_worry_level //= 3
                else:
                    item_worry_level %= lcm
                if item_worry_level % monkey.test_divisible == 0:
                    monkeys[monkey.true_throw].items.append(item_worry_level)
                else:
                    monkeys[monkey.false_throw].items.append(item_worry_level)

                monkey.processed_items += 1

    first, second = sorted(
        [monkey.processed_items for monkey in monkeys], reverse=True
    )[:2]
    return first * second


def part_one(path: str | None = None) -> int:
    with open(path if path else "input/day11_input") as f:
        return run(parse_monkeys(f.read()), 20)


def part_two(path: str | None = None):
    with open(path if path else "input/day11_input") as f:
        return run(parse_monkeys(f.read()), 10000)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day11_test_input"), 10605)

    def test_part_two(self):
        self.assertEqual(part_two("input/day11_test_input"), 2713310158)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
