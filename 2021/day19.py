import itertools
import unittest

DAY = 19

# https://reddit.com/r/adventofcode/comments/rjpf7f/2021_day_19_solutions/hp51a0b
Vector = tuple[int, int, int]
Matrix = tuple[Vector, Vector, Vector]


def parse_input(input_path: str) -> list[set[Vector]]:
    with open(input_path) as f:
        scanners = []
        scanner_parts = f.read().split("\n\n")
        for part in scanner_parts:
            new_scanner: set[Vector] = set()
            for line in part.splitlines():
                if not line.startswith("---"):
                    new_scanner.add(tuple(int(num) for num in line.strip().split(",")))
            scanners.append(new_scanner)
        return scanners


# https://en.wikipedia.org/wiki/Dot_product
def dot_product(va: Vector, vb: Vector) -> int:
    return sum(a * b for a, b in zip(va, vb))


# https://en.wikipedia.org/wiki/Cross_product
def cross_product(va: Vector, vb: Vector) -> Vector:
    a1, a2, a3 = va
    b1, b2, b3 = vb
    return a2 * b3 - a3 * b2, a3 * b1 - a1 * b3, a1 * b2 - a2 * b1


def rotate_matrixes() -> list[Matrix]:
    matrixes = []
    bases = [x for y in [-1, 1] for x in [[y, 0, 0], [0, y, 0], [0, 0, y]]]
    for i, j in itertools.permutations(bases, 2):
        if dot_product(i, j) == 0:  # i and j are orthogonal
            matrixes.append((i, j, cross_product(i, j)))
    return matrixes


def rotate_vector(matrix: Matrix, vector: Vector) -> Vector:
    result = []
    for i in range(len(matrix)):
        current_value = 0
        for j in range(len(vector)):
            current_value += vector[j] * matrix[j][i]
        result.append(current_value)
    return tuple(result)


def distance_diff(beacon_a: Vector, beacon_b: Vector) -> Vector:
    a_x, a_y, a_z = beacon_a
    b_x, b_y, b_z = beacon_b
    return a_x - b_x, a_y - b_y, a_z - b_z


def apply_diff(beacon: Vector, diff: Vector) -> Vector:
    x, y, z = beacon
    dx, dy, dz = diff
    return x + dx, y + dy, z + dz


def align(
    scanner_a: set[Vector], scanner_b: set[Vector]
) -> tuple[set[Vector], Vector] | None:
    for rotate_matrix in rotate_matrixes():
        rotated_b = [rotate_vector(rotate_matrix, p) for p in scanner_b]

        for beacon_a in scanner_a:
            for beacon_b in rotated_b:
                diff = distance_diff(beacon_a, beacon_b)
                new_beacons = {apply_diff(p, diff) for p in rotated_b}
                if len(scanner_a.intersection(new_beacons)) >= 12:
                    return new_beacons, diff
    return None


def part_one(input_path: str) -> int:
    scanners = parse_input(input_path)
    aligned_indexes = {
        0,
    }
    new_aligned_indexes = {
        0,
    }
    beacons = scanners[0]
    while new_aligned_indexes:
        next_new_indexes = set()
        for aligned_index in new_aligned_indexes:
            for another_index in range(len(scanners)):
                if (
                    another_index != aligned_index
                    and another_index not in aligned_indexes
                ):
                    result = align(scanners[aligned_index], scanners[another_index])
                    if result:
                        aligned_indexes.add(another_index)
                        next_new_indexes.add(another_index)
                        beacons |= result[0]
                        scanners[another_index] = result[0]

        new_aligned_indexes = next_new_indexes

    return len(beacons)


def manhattan_distance(va: Vector, vb: Vector) -> int:
    return sum(abs(a - b) for a, b in zip(va, vb))


def part_two(input_path: str) -> int:
    scanners = parse_input(input_path)
    aligned_indexes = {
        0,
    }
    new_aligned_indexes = {
        0,
    }
    scanner_positions = {
        (0, 0, 0),
    }
    while new_aligned_indexes:
        next_new_indexes = set()
        for aligned_index in new_aligned_indexes:
            for another_index in range(len(scanners)):
                if (
                    another_index != aligned_index
                    and another_index not in aligned_indexes
                ):
                    result = align(scanners[aligned_index], scanners[another_index])
                    if result:
                        aligned_indexes.add(another_index)
                        next_new_indexes.add(another_index)
                        scanners[another_index] = result[0]
                        scanner_positions.add(result[1])

        new_aligned_indexes = next_new_indexes

    return max(
        manhattan_distance(a, b)
        for a, b in itertools.combinations(scanner_positions, 2)
    )


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 79)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 3621)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
