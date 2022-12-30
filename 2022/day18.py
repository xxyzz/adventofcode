import itertools
import math
import unittest
from collections import defaultdict, deque

DAY = 18


def part_one(path: str) -> int:
    with open(path) as f:
        lava_cubes = {}
        for line in f:
            cube = tuple(map(int, line.strip().split(",")))
            lava_cubes[cube] = 6

        return cube_surfaces(lava_cubes)


def cube_surfaces(cubes):
    for cube1, cube2 in itertools.combinations(cubes, 2):
        if check_adjacent(cube1, cube2):
            cubes[cube1] -= 1
            cubes[cube2] -= 1

    return sum(cubes.values())


def check_adjacent(cube1, cube2) -> bool:
    x1, y1, z1 = cube1
    x2, y2, z2 = cube2
    if y1 == y2 and z1 == z2 and abs(x1 - x2) == 1:
        return True
    if x1 == x2 and y1 == y2 and abs(z1 - z2) == 1:
        return True
    if x1 == x2 and z1 == z2 and abs(y1 - y2) == 1:
        return True
    return False


def adjacent_cube(point):
    x, y, z = point
    for dx, dy, dz in [
        (0, 0, 1),
        (0, 0, -1),
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
    ]:
        yield x + dx, y + dy, z + dz


def cube_range(cubes):
    min_x = math.inf
    max_x = -math.inf
    min_y = math.inf
    max_y = -math.inf
    min_z = math.inf
    max_z = -math.inf
    for x, y, z in cubes:
        min_x = min(min_x, x)
        max_x = max(max_x, x)
        min_y = min(min_y, y)
        max_y = max(max_y, y)
        min_z = min(min_z, z)
        max_z = max(max_z, z)
    return range(min_x, max_x + 1), range(min_y, max_y + 1), range(min_z, max_z + 1)


def part_two(path: str) -> int:
    lava_cubes: set[tuple[int, int, int]] = set()
    with open(path) as f:
        for line in f:
            cube = tuple(map(int, line.strip().split(",")))
            lava_cubes.add(cube)

    surfaces = cube_surfaces({cube: 6 for cube in lava_cubes})
    x_range, y_range, z_range = cube_range(lava_cubes)
    bounding_box = {
        (x, y, z) for x, y, z in itertools.product(x_range, y_range, z_range)
    }
    visited = set()
    for cube in bounding_box - lava_cubes:
        if cube not in visited:
            touched_faces, visited_cubes = escape(
                cube, lava_cubes, x_range, y_range, z_range
            )
            visited |= visited_cubes
            surfaces -= touched_faces
    return surfaces


# https://github.com/mebeim/aoc/tree/master/2022#day-18---boiling-boulders
def escape(start, lava_cubes, x_range, y_range, z_range):
    q = deque([start])
    visited = set()
    touched_faces = 0
    while q:
        cube = q.popleft()
        if cube in visited:
            continue
        visited.add(cube)
        x, y, z = cube

        if x not in x_range or y not in y_range or z not in z_range:
            return 0, visited

        for next_cube in adjacent_cube(cube):
            if next_cube in lava_cubes:
                touched_faces += 1
            elif next_cube not in visited:
                q.append(next_cube)

    return touched_faces, visited


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 64)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 58)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
