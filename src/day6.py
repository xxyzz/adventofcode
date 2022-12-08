import unittest


def part_one() -> int:
    with open("src/day6_input") as f:
        return find_marker(f.read().strip(), 4)
    return -1


def part_two() -> int:
    with open("src/day6_input") as f:
        return find_marker(f.read().strip(), 14)
    return -1


def find_marker(text: str, marker_len: int) -> int:
    for i in range(len(text) - marker_len + 1):
        if len(set(text[i : i + marker_len])) == marker_len:
            return i + marker_len
    return -1


class TestDevice(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7)
        self.assertEqual(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5)
        self.assertEqual(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6)
        self.assertEqual(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10)
        self.assertEqual(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11)

    def test_part_two(self):
        self.assertEqual(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19)
        self.assertEqual(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23)
        self.assertEqual(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23)
        self.assertEqual(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29)
        self.assertEqual(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26)


if __name__ == "__main__":
    print(part_two())
