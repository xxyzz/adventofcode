import functools
import itertools
import math
import unittest
from collections import defaultdict

DAY = 16


def parse_input(path: str) -> tuple[dict[str, int], dict[tuple[str, str], int]]:
    with open(path) as f:
        valves = set()
        valve_rates = {}
        distances = {}
        for line in f:
            label_rate_str, tunnel_str = line.strip().split(";")
            label_rate_split = label_rate_str.split()
            label = label_rate_split[1]
            rate = int(label_rate_split[-1].split("=")[-1])
            tunnels = [l.removesuffix(",") for l in tunnel_str.split()[4:]]
            valves.add(label)
            if rate > 0:
                valve_rates[label] = rate  # only find openable valves
            distances[label, label] = 0
            for next_valve in tunnels:
                distances[label, next_valve] = 1
                distances[next_valve, label] = 1

        # https://en.wikipedia.org/wiki/Floyd-Warshall_algorithm
        for k, i, j in itertools.permutations(valves, 3):
            distances[i, j] = min(
                distances.get((i, j), math.inf),
                distances.get((i, k), math.inf) + distances.get((k, j), math.inf),
            )
        return valve_rates, distances


def dfs(valve, remain_time, distances, remain_valves, opened_valves):
    for next_valve in remain_valves:
        new_time = remain_time - distances[valve, next_valve] - 1
        if new_time <= 0:
            continue
        yield from dfs(
            next_valve,
            new_time,
            distances,
            remain_valves - {next_valve},
            opened_valves | {next_valve: new_time},
        )
    yield opened_valves


def calculate_pressure(
    opened_valves: dict[str, int], valve_rates: dict[str, int] = {}
) -> int:
    return sum(valve_rates[valve] * time for valve, time in opened_valves.items())


def part_one(path: str) -> int:
    valve_rates, distances = parse_input(path)
    return max(
        map(
            functools.partial(calculate_pressure, valve_rates=valve_rates),
            dfs("AA", 30, distances, frozenset(valve_rates), {}),
        )
    )


def part_two(path: str) -> int:
    valve_rates, distances = parse_input(path)
    released_pressure = defaultdict(int)
    for paths in dfs("AA", 26, distances, frozenset(valve_rates), {}):
        paths_set = frozenset(paths)
        pressure = calculate_pressure(paths, valve_rates)
        released_pressure[paths_set] = max(
            pressure, released_pressure.get(paths_set, 0)
        )

    max_flow_rate = 0
    for (path1, rate1), (path2, rate2) in itertools.combinations(
        released_pressure.items(), 2
    ):
        if path1 & path2:
            continue
        max_flow_rate = max(max_flow_rate, rate1 + rate2)
    return max_flow_rate


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 1651)

    def test_part_two(self):
        self.assertEqual(part_two(f"input/day{DAY}_test_input"), 1707)


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    print(part_two(f"input/day{DAY}_input"))
