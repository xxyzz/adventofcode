import functools
import re
import unittest
from collections import deque

DAY = 19

MAX_GEODE = 0


def part_one(path: str) -> int:
    with open(path) as f:
        blueprints = []
        for line in f:
            blueprint = []
            robot_costs = line.strip()[line.index(":") :].split(".")
            for robot_cost in filter(None, robot_costs):
                blueprint.append(
                    tuple(
                        map(lambda x: int(x.group()), re.finditer(r"\d+", robot_cost))
                    )
                )
            blueprints.append(blueprint)

        result = 0
        global MAX_GEODE
        for blueprint_id, blueprint in enumerate(blueprints, start=1):
            MAX_GEODE = 0
            geodes = dfs(blueprint, 24, 0, 1, 0, 0, 0, 0, 0, 0)
            # print(f"{blueprint_id=} {geodes=}")
            result += blueprint_id * geodes
        return result


def dfs(
    blueprint,
    minutes,
    ore,
    ore_robots,
    clay,
    clay_robots,
    obsidian,
    obsidian_robots,
    geode,
    geode_robots,
):
    global MAX_GEODE
    if max_produceable(geode, geode_robots, minutes) <= MAX_GEODE:
        return

    ore_robot_ore = blueprint[0][0]
    clay_robot_ore = blueprint[1][0]
    obsidian_robot_ore, obsidian_robot_clay = blueprint[2]
    geode_robot_ore, geode_robot_obsidian = blueprint[3]
    max_ore_needed = max(
        ore_robot_ore, clay_robot_ore, obsidian_robot_ore, geode_robot_ore
    )
    new_ore = ore + ore_robots  # Damn, can't use += at here
    new_clay = clay + clay_robots
    new_obsidian = obsidian + obsidian_robots
    new_geode = geode + geode_robots
    minutes -= 1
    if minutes <= 0:
        MAX_GEODE = max(MAX_GEODE, new_geode)
        return

    if ore >= ore_robot_ore and ore_robots < max_ore_needed:
        dfs(
            blueprint,
            minutes,
            new_ore - ore_robot_ore,
            ore_robots + 1,
            new_clay,
            clay_robots,
            new_obsidian,
            obsidian_robots,
            new_geode,
            geode_robots,
        )
    if ore >= clay_robot_ore and clay_robots < obsidian_robot_clay:
        dfs(
            blueprint,
            minutes,
            new_ore - clay_robot_ore,
            ore_robots,
            new_clay,
            clay_robots + 1,
            new_obsidian,
            obsidian_robots,
            new_geode,
            geode_robots,
        )
    if (
        ore >= obsidian_robot_ore
        and clay >= obsidian_robot_clay
        and obsidian_robots < geode_robot_obsidian
    ):
        dfs(
            blueprint,
            minutes,
            new_ore - obsidian_robot_ore,
            ore_robots,
            new_clay - obsidian_robot_clay,
            clay_robots,
            new_obsidian,
            obsidian_robots + 1,
            new_geode,
            geode_robots,
        )
    if ore >= geode_robot_ore and obsidian >= geode_robot_obsidian:
        dfs(
            blueprint,
            minutes,
            new_ore - geode_robot_ore,
            ore_robots,
            new_clay,
            clay_robots,
            new_obsidian - geode_robot_obsidian,
            obsidian_robots,
            new_geode,
            geode_robots + 1,
        )

    if (
        ore < max_ore_needed
        or (clay_robots and clay < obsidian_robot_clay)
        or (obsidian_robots and obsidian < geode_robot_obsidian)
    ):
        dfs(
            blueprint,
            minutes,
            new_ore,
            ore_robots,
            new_clay,
            clay_robots,
            new_obsidian,
            obsidian_robots,
            new_geode,
            geode_robots,
        )

    return MAX_GEODE


def max_produceable(current, robots, minutes):
    return current + robots * minutes + minutes * (minutes - 1) // 2


def part_two(path: str) -> int:
    with open(path) as f:
        pass


class Test(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(f"input/day{DAY}_test_input"), 33)

    # def test_part_two(self):
    #     self.assertEqual(part_two(f"input/day{DAY}_test_input"), )


if __name__ == "__main__":
    print(part_one(f"input/day{DAY}_input"))
    # print(part_two(f"input/day{DAY}_input"))
