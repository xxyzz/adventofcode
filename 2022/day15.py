import re
import unittest

DAY = 15


def part_one(path: str, target_y: int = 2000000) -> int:
    sensor_with_dist = set()
    beacons = set()
    sensors = set()
    not_beacons = set()
    with open(path) as f:
        for line in f:
            numbers = [int(m.group()) for m in re.finditer(r"-?\d+", line)]
            sensors.add((numbers[0], numbers[1]))
            beacons.add((numbers[2], numbers[3]))
            min_dist = abs(numbers[0] - numbers[2]) + abs(numbers[1] - numbers[3])
            sensor_with_dist.add(((numbers[0], numbers[1]), min_dist))

        for (x, y), min_dist in sensor_with_dist:
            if y - min_dist <= target_y <= y + min_dist:
                dy = abs(target_y - y)
                dx = min_dist - dy
                for n_x in range(x - dx, x + dx + 1):
                    new_point = (n_x, target_y)
                    if new_point not in sensors and new_point not in beacons:
                        not_beacons.add(new_point)

    return len(not_beacons)


def part_two(path: str, limit: int = 4000000) -> int:
    sensor_with_dist = set()
    with open(path) as f:
        for line in f:
            numbers = [int(m.group()) for m in re.finditer(r"-?\d+", line)]
            min_dist = abs(numbers[0] - numbers[2]) + abs(numbers[1] - numbers[3])
            sensor_with_dist.add(((numbers[0], numbers[1]), min_dist))

        for (x, y), dist in sensor_with_dist:
            for nx, ny in outer_edge(x, y, dist, limit):
                for (sx, sy), sdist in sensor_with_dist:
                    if sx != x and abs(nx - sx) + abs(ny - sy) <= sdist:
                        break
                else:
                    return nx * 4000000 + ny

        return -1


def outer_edge(x: int, y: int, dist: int, limit: int) -> list[tuple[int, int]]:
    edge_points = []
    for dx in range(-dist - 1, dist + 2):
        dy_abs = dist + 1 - dx
        for dy in [-dy_abs, dy_abs]:
            nx = x + dx
            ny = y + dy
            if 0 <= nx <= limit and 0 <= ny <= limit:
                edge_points.append((nx, ny))
    return edge_points


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input", 10), 26)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input", 20), 56000011)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
