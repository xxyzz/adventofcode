import unittest


def part_one(input_path: str) -> int:
    with open(input_path) as f:
        result = 0
        for line in f:
            outputs = line.split("|")[-1].split()
            result += sum(len(output) in [2, 4, 3, 7] for output in outputs)
        return result


def part_two(input_path: str) -> int:
    with open(input_path) as f:
        result = 0
        pattern_segments = []
        output_segments = []
        for line in f:
            patterns_str, outputs_str = line.split("|")
            pattern_segments.append(
                [frozenset(pattern) for pattern in patterns_str.split()]
            )
            output_segments.append(
                [frozenset(pattern) for pattern in outputs_str.split()]
            )

        for patterns, outputs in zip(pattern_segments, output_segments):
            pattern_to_digit: dict[frozenset[str], int] = {}
            for pattern in patterns:
                match len(pattern):
                    case 2:
                        pattern_to_digit[pattern] = 1
                    case 3:
                        pattern_to_digit[pattern] = 7
                    case 4:
                        pattern_to_digit[pattern] = 4
                    case 7:
                        pattern_to_digit[pattern] = 8

            digit_to_pattern = {
                digit: pattern for pattern, digit in pattern_to_digit.items()
            }

            for pattern in patterns:
                match len(pattern):
                    case 5:
                        if len(pattern & digit_to_pattern[7]) == 3:
                            pattern_to_digit[pattern] = 3
                        elif len(pattern & digit_to_pattern[4]) == 3:
                            pattern_to_digit[pattern] = 5
                        else:
                            pattern_to_digit[pattern] = 2
                    case 6:
                        if len(pattern & digit_to_pattern[4]) == 4:
                            pattern_to_digit[pattern] = 9
                        elif len(pattern & digit_to_pattern[7]) == 3:
                            pattern_to_digit[pattern] = 0
                        else:
                            pattern_to_digit[pattern] = 6

            value = 0
            for output in outputs:
                value = value * 10 + pattern_to_digit[output]
            result += value
        return result


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one("input/day8_test_input"), 26)

    def test_part_two(self):
        self.assertEqual(part_two("input/day8_test_input"), 61229)


if __name__ == "__main__":
    print(part_one("input/day8_input"))
    print(part_two("input/day8_input"))
