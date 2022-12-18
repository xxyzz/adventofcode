import unittest

DAY = 12


def parse(f):
    connected_caves = defaultdict(list)
    for line in f:
        cave_a, cave_b = line.strip().split("-")
        if cave_a == "end" or cave_b == "start":
            cave_a, cave_b = cave_b, cave_a
        connected_caves[cave_a].append(cave_b)
        if cave_a != "start" and cave_b != "end":
            connected_caves[cave_b].append(cave_a)

    return connected_caves


# https://www.reddit.com/r/adventofcode/comments/rehj2r/comment/ho7x83o
def dfs(connected_caves, cave, visited_caves, allow_revisit):
    if cave == "end":
        return 1
    if cave in visited_caves and cave.islower():
        if allow_revisit:
            allow_revisit = False
        else:
            return 0

    return sum(
        dfs(connected_caves, next_cave, visited_caves | {cave}, allow_revisit)
        for next_cave in connected_caves[cave]
    )


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        connected_caves = parse(f)
        return dfs(connected_caves, "start", set(), False)


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        connected_caves = parse(f)
        return dfs(connected_caves, "start", set(), True)


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input1"), 10)
        self.assertEqual(part_one(f"input/day{DAY}_test_input2"), 19)
        self.assertEqual(part_one(f"input/day{DAY}_test_input3"), 226)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input1"), 36)
        self.assertEqual(part_two(f"input/day{DAY}_test_input2"), 103)
        self.assertEqual(part_two(f"input/day{DAY}_test_input3"), 3509)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
