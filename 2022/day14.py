import unittest

DAY = 14

def part_one(path: str) -> int:
    cave_map = set()
    sand_count = 0
    max_y = 0
    with open(path) as f:
        for line in f:
            points = [tuple(map(int, point.split(","))) for point in line.strip().split("->")]
            cave_map.add(points[0])
            if points[0][1] > max_y:
                max_y = points[0][1]
            for i in range(1, len(points)):
                prev_p_x, prev_p_y = points[i - 1]
                p_x, p_y = points[i]
                if prev_p_x == p_x:
                    if prev_p_y > p_y:
                        prev_p_y, p_y = p_y, prev_p_y
                    for y in range(prev_p_y, p_y + 1):
                        cave_map.add((prev_p_x, y))
                elif prev_p_y == p_y:
                    if prev_p_x > p_x:
                        prev_p_x, p_x = p_x, prev_p_x
                    for x in range(prev_p_x, p_x + 1):
                        cave_map.add((x, p_y))
                if p_y > max_y:
                    max_y = p_y

        sand_x = 500
        sand_y = 0
        while True:
            if (sand_x, sand_y + 1) not in cave_map:
                sand_y += 1
                if sand_y >= max_y:
                    break
            else:
                if (sand_x - 1, sand_y + 1) not in cave_map:
                    sand_x -= 1
                    sand_y += 1
                elif (sand_x + 1, sand_y + 1) not in cave_map:
                    sand_x += 1
                    sand_y += 1
                else:
                    cave_map.add((sand_x, sand_y))
                    sand_count += 1
                    sand_x = 500
                    sand_y = 0
        return sand_count


def part_two(path: str) -> int:
    cave_map = set()
    sand_count = 0
    max_y = 0
    bottom = 0
    with open(path) as f:
        for line in f:
            points = [tuple(map(int, point.split(","))) for point in line.strip().split("->")]
            cave_map.add(points[0])
            if points[0][1] > max_y:
                max_y = points[0][1]
            for i in range(1, len(points)):
                prev_p_x, prev_p_y = points[i - 1]
                p_x, p_y = points[i]
                if prev_p_x == p_x:
                    if prev_p_y > p_y:
                        prev_p_y, p_y = p_y, prev_p_y
                    for y in range(prev_p_y, p_y + 1):
                        cave_map.add((prev_p_x, y))
                elif prev_p_y == p_y:
                    if prev_p_x > p_x:
                        prev_p_x, p_x = p_x, prev_p_x
                    for x in range(prev_p_x, p_x + 1):
                        cave_map.add((x, p_y))
                if p_y > max_y:
                    max_y = p_y

        bottom = max_y + 2
        sand_x = 500
        sand_y = 0
        while True:
            if sand_y == bottom:
                cave_map.add((sand_x, sand_y - 1))
                sand_count += 1
                sand_x = 500
                sand_y = 0
            elif (sand_x, sand_y + 1) not in cave_map:
                sand_y += 1
            else:
                if (sand_x - 1, sand_y + 1) not in cave_map:
                    sand_x -= 1
                    sand_y += 1
                elif (sand_x + 1, sand_y + 1) not in cave_map:
                    sand_x += 1
                    sand_y += 1
                else:
                    cave_map.add((sand_x, sand_y))
                    sand_count += 1
                    if sand_x == 500 and sand_y == 0:
                        break
                    sand_x = 500
                    sand_y = 0
        return sand_count


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 24)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 93)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
