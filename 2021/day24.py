from collections import deque

DAY = 24

# https://reddit.com/r/adventofcode/comments/rnejv5/2021_day_24_solutions/hps5hgw
def part_one(input_path: str) -> int:
    stack: deque[list[int]] = deque()
    result = [""] * 14
    ins_index = 0
    result_index = 0
    with open(input_path) as f:
        values = [result_index]
        for line in f:
            ins = line.strip().split()
            if ins_index in [4, 5, 15]:
                values.append(int(ins[-1]))
            elif ins_index == 17:
                if values[1] == 1:
                    stack.append(values)
                elif values[1] == 26:
                    prev_index, _, _, c = stack.pop()
                    _, _, b, _ = values
                    if b + c > 0:
                        result[result_index] = "9"
                        result[prev_index] = str(9 - b - c)
                    else:
                        result[prev_index] = "9"
                        result[result_index] = str(9 + b + c)
                result_index += 1
                values = [result_index]
            ins_index += 1
            ins_index %= 18
    return int("".join(result))


def part_two(input_path: str) -> int:
    stack: deque[list[int]] = deque()
    result = [""] * 14
    ins_index = 0
    result_index = 0
    with open(input_path) as f:
        values = [result_index]
        for line in f:
            ins = line.strip().split()
            if ins_index in [4, 5, 15]:
                values.append(int(ins[-1]))
            elif ins_index == 17:
                if values[1] == 1:
                    stack.append(values)
                elif values[1] == 26:
                    prev_index, _, _, c = stack.pop()
                    _, _, b, _ = values
                    if b + c > 0:
                        result[result_index] = str(1 + b + c)
                        result[prev_index] = "1"
                    else:
                        result[prev_index] = str(1 - b - c)
                        result[result_index] = "1"
                result_index += 1
                values = [result_index]
            ins_index += 1
            ins_index %= 18
    return int("".join(result))


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
