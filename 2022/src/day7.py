import io
import unittest
from collections import deque
from dataclasses import dataclass
from typing import Any, TextIO


@dataclass
class File:
    name: str
    size: int


@dataclass
class Dir:
    name: str
    contents: list[Any]


def create_fs_tree(f: TextIO) -> Dir:
    pwd: deque[Dir] = deque()
    for line in f:
        output = line.strip().split()
        if not output:
            continue
        match output[0]:
            case "$":
                if output[1] == "cd":
                    if output[2] == "..":
                        pwd.pop()
                    else:
                        if pwd:
                            for d in pwd[-1].contents:
                                if isinstance(d, Dir) and d.name == output[2]:
                                    pwd.append(d)
                                    break
                        else:
                            pwd.append(Dir(output[2], []))
            case "dir":
                pwd[-1].contents.append(Dir(output[1], []))
            case _:
                pwd[-1].contents.append(File(output[1], int(output[0])))

    return pwd[0]


def calculate_folder_size(folder: Dir, folder_sizes: list[int]) -> int:
    size = 0
    for content in folder.contents:
        if isinstance(content, File):
            size += content.size
        else:
            size += calculate_folder_size(content, folder_sizes)

    folder_sizes.append(size)
    return size


def part_one(path: str | None = None) -> int:
    with open(path if path else "src/day7_input") as f:
        root = create_fs_tree(f)

        folder_sizes: list[int] = []
        calculate_folder_size(root, folder_sizes)
        return sum(filter(lambda x: x <= 100000, folder_sizes))


def part_two(path: str | None = None) -> int:
    with open(path if path else "src/day7_input") as f:
        root = create_fs_tree(f)

        folder_sizes: list[int] = []
        calculate_folder_size(root, folder_sizes)
        total_disk_size = 70000000
        folder_sizes.sort()
        root_size = folder_sizes[-1]
        free_size = total_disk_size - root_size
        for size in folder_sizes:
            if size + free_size >= 30000000:
                return size
        return -1


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("src/day7_test_input"), 95437)

    def test_part_two(self):
        self.assertEqual(part_two("src/day7_test_input"), 24933642)


if __name__ == "__main__":
    print(part_two())
