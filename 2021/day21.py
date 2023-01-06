import functools
import itertools
import unittest

DAY = 21


def parse_input(input_path: str) -> list[int]:
    with open(input_path) as f:
        return list(int(line.strip().split(":")[-1]) for line in f)


# https://github.com/norvig/pytudes/blob/main/ipynb/Advent-2021.ipynb
NORMAL_DICE = itertools.cycle(range(1, 101))


def part_one(input_path: str) -> int:
    player_positions = parse_input(input_path)
    player_scores = [0, 0]
    for turn in itertools.count(1):
        player = (turn - 1) % 2
        steps = next(NORMAL_DICE) + next(NORMAL_DICE) + next(NORMAL_DICE)
        next_pos = (player_positions[player] + steps - 1) % 10 + 1
        player_positions[player] = next_pos
        player_scores[player] += next_pos
        if player_scores[player] >= 1000:
            return min(player_scores) * 3 * turn

    return 0


QUANTUM_DIE_SUM = list(map(sum, itertools.product([1, 2, 3], repeat=3)))  # 27 universes


@functools.cache
def roll_quantim_die(pos1: int, pos2: int, score1: int, score2: int) -> tuple[int, int]:
    wins = 0
    loses = 0
    if score1 >= 21:
        wins += 1
    elif score2 >= 21:
        loses += 1
    else:
        for steps in QUANTUM_DIE_SUM:
            new_pos1 = (pos1 + steps - 1) % 10 + 1
            roll_losses, roll_wins = roll_quantim_die(  # player 2 rolls
                pos2, new_pos1, score2, score1 + new_pos1
            )
            wins += roll_wins
            loses += roll_losses
    return wins, loses


def part_two(input_path: str) -> int:
    pos1, pos2 = parse_input(input_path)
    return max(roll_quantim_die(pos1, pos2, 0, 0))


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 739785)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 444356092776315)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
