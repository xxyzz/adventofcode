import unittest
from collections import deque

DAY = 10


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        points = 0
        for line in f:
            stack = deque()
            for c in line.strip():
                match c:
                    case "(":
                        stack.append(")")
                    case "[":
                        stack.append("]")
                    case "{":
                        stack.append("}")
                    case "<":
                        stack.append(">")
                    case ")":
                        if stack[-1] != ")":
                            points += 3
                            break
                        else:
                            stack.pop()
                    case "]":
                        if stack[-1] != "]":
                            points += 57
                            break
                        else:
                            stack.pop()
                    case "}":
                        if stack[-1] != "}":
                            points += 1197
                            break
                        else:
                            stack.pop()
                    case ">":
                        if stack[-1] != ">":
                            points += 25137
                            break
                        else:
                            stack.pop()
        return points


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        points = []
        for line in f:
            stack = deque()
            point = 0
            for c in line.strip():
                match c:
                    case "(":
                        stack.append(")")
                    case "[":
                        stack.append("]")
                    case "{":
                        stack.append("}")
                    case "<":
                        stack.append(">")
                    case ")":
                        if stack[-1] != ")":
                            break
                        else:
                            stack.pop()
                    case "]":
                        if stack[-1] != "]":
                            break
                        else:
                            stack.pop()
                    case "}":
                        if stack[-1] != "}":
                            break
                        else:
                            stack.pop()
                    case ">":
                        if stack[-1] != ">":
                            break
                        else:
                            stack.pop()
            else:
                while stack:
                    c = stack.pop()
                    point *= 5
                    match c:
                        case ")":
                            point += 1
                        case "]":
                            point += 2
                        case "}":
                            point += 3
                        case ">":
                            point += 4
                points.append(point)
        points.sort()
        return points[len(points) // 2]


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 26397)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 288957)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
