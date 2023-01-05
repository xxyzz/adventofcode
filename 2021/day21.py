import unittest

DAY = 21

def parse_input(input_path: str) -> list[int]:
    with open(input_path) as f:
        return list(int(line.strip().split(":")[-1]) for line in f)

def roll_dice(dice_num: int) -> tuple[int, int]:
    sum_roll = 0
    for _ in range(3):
        dice_num = (dice_num % 100) + 1
        sum_roll += dice_num
    return dice_num, sum_roll

def part_one(input_path: str) -> int:
    player1_pos, player2_pos = parse_input(input_path)
    player1_score = 0
    player2_score = 0
    dice_num = 0
    rolled_num = 0
    while True:
        dice_num, sum_roll = roll_dice(dice_num)
        rolled_num += 3
        player1_pos = (player1_pos + sum_roll - 1) % 10 + 1
        player1_score += player1_pos
        if player1_score >= 1000:
            return player2_score * rolled_num

        dice_num, sum_roll = roll_dice(dice_num)
        rolled_num += 3
        player2_pos = (player2_pos + sum_roll - 1) % 10 + 1
        player2_score += player2_pos
        if player2_score >= 1000:
            return player1_score * rolled_num
    return 0


def part_two(input_path: str) -> int:
    pass

class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 739785)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
