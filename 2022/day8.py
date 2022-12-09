import unittest
from typing import TextIO


def part_one(path: str | None = None) -> int:
    with open(path if path else "input/day8_input") as f:
        trees = scan_trees(f)
        visible_trees = 0
        row_num = len(trees)
        colum_num = len(trees[0])

        # edge trees
        visible_trees += 2 * row_num
        visible_trees += 2 * (colum_num - 2)

        for i in range(1, row_num - 1):
            for j in range(1, colum_num - 1):
                visible = True
                current_tree = trees[i][j]
                for k in range(i - 1, -1, -1):  # above
                    if trees[k][j] >= current_tree:
                        visible = False
                        break
                if not visible:
                    visible = True
                    for k in range(i + 1, row_num):  # bottom
                        if trees[k][j] >= current_tree:
                            visible = False
                            break
                if not visible:
                    visible = True
                    for k in range(j - 1, -1, -1):  # left
                        if trees[i][k] >= current_tree:
                            visible = False
                            break
                if not visible:
                    visible = True
                    for k in range(j + 1, colum_num):  # right
                        if trees[i][k] >= current_tree:
                            visible = False
                            break
                if visible:
                    visible_trees += 1
        return visible_trees


def scan_trees(f: TextIO) -> list[list[int]]:
    return [[int(v) for v in line.strip()] for line in f]


def part_two(path: str | None = None) -> int:
    with open(path if path else "input/day8_input") as f:
        trees = scan_trees(f)
        max_scenic = 0
        row_num = len(trees)
        colum_num = len(trees[0])

        for i in range(1, row_num - 1):
            for j in range(1, colum_num - 1):
                scenic_score = 1
                current_tree = trees[i][j]

                current_visible_trees = 0
                for k in range(i - 1, -1, -1):  # above
                    current_visible_trees += 1
                    if trees[k][j] >= current_tree:
                        break
                scenic_score *= current_visible_trees

                current_visible_trees = 0
                for k in range(i + 1, row_num):  # bottom
                    current_visible_trees += 1
                    if trees[k][j] >= current_tree:
                        break
                scenic_score *= current_visible_trees

                current_visible_trees = 0
                for k in range(j - 1, -1, -1):  # left
                    current_visible_trees += 1
                    if trees[i][k] >= current_tree:
                        break
                scenic_score *= current_visible_trees

                current_visible_trees = 0
                for k in range(j + 1, colum_num):  # right
                    current_visible_trees += 1
                    if trees[i][k] >= current_tree:
                        break
                scenic_score *= current_visible_trees

                if scenic_score > max_scenic:
                    max_scenic = scenic_score

    return max_scenic


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day8_test_input"), 21)

    def test_part_two(self):
        self.assertEqual(part_two("input/day8_test_input"), 8)


if __name__ == "__main__":
    print(part_one())
    print(part_two())
