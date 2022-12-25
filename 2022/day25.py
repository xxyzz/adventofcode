import unittest
from itertools import zip_longest

DAY = 25

def SNAFU_to_int(num: str) -> int:
    match num:
        case "=":
            return -2
        case "-":
            return -1
        case None:
            return 0
        case _:
            return int(num)

def SNAFU_add(num_a: str, num_b: str) -> str:
    sum_num = ""
    carry = 0
    for d_a_str, d_b_str in zip_longest(reversed(num_a), reversed(num_b)):
        d_a = SNAFU_to_int(d_a_str)
        d_b = SNAFU_to_int(d_b_str)
        temp_sum = d_a + d_b + carry
        current_sum = ""
        match temp_sum:
            case 5:
                carry = 1
                current_sum = "0"
            case 4:
                carry = 1
                current_sum = "-"
            case 3:
                carry = 1
                current_sum = "="
            case -1:
                carry = 0
                current_sum = "-"
            case -2:
                carry = 0
                current_sum = "="
            case -3:
                carry = -1
                current_sum = "2"
            case -4:
                carry = -1
                current_sum = "1"
            case -5:
                carry = -1
                current_sum = "0"
            case _:
                carry = 0
                current_sum = str(temp_sum)
        sum_num += current_sum

    if carry == -2:
        sum_num += "="
    elif carry == "-1":
        sum_num += "-"
    elif carry != 0:
        sum_num += str(carry)

    return sum_num[::-1]


def part_one(path: str) -> str:
    with open(path) as f:
        result = ""
        for line in f:
            result = SNAFU_add(result, line.strip())
        return result
        

def part_two(path: str) -> str:
    with open(path) as f:
        pass


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), "2=-1=0")

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
