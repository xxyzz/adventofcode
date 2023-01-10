import heapq
import unittest
from dataclasses import dataclass
from typing import Any, Self  # Python 3.11

DAY = 23

# https://github.com/benediktwerner/AdventOfCode/blob/master/2021/day23/sol.py
NON_STOP_HALLWAY = [2, 4, 6, 8]


@dataclass(frozen=True)
class State:
    energy: int
    rooms: tuple[tuple[int, ...], ...]
    hallway: tuple[int | None, ...] = (None,) * 11

    def __lt__(self: Self, other: Self):
        return self.energy < other.energy

    @property
    def fingerprint(self):
        return self.hallway + self.rooms

    @property
    def is_done(self):
        return all(hallway is None for hallway in self.hallway) and all(
            room == room_index
            for room_index, rooms in enumerate(self.rooms)
            for room in rooms
        )


def insert_tuple(origin: tuple[Any, ...], index: int, new: Any) -> tuple[Any, ...]:
    return origin[:index] + (new,) + origin[index + 1 :]


def solve(start: State) -> int:
    room_size = len(start.rooms[0])
    pq = [start]
    visited = set()
    while pq:
        state = heapq.heappop(pq)
        if state.is_done:
            return state.energy
        if state.fingerprint in visited:
            continue
        visited.add(state.fingerprint)

        # move amphipod at the top room to hallway
        for room_index, room in enumerate(state.rooms):
            if room and not all(r == room_index for r in room):
                amphipod_type = room[-1]  # 0 - 3
                for end, step in [(11, 1), (-1, -1)]:  # move to right and left
                    for hallway_index in range(
                        NON_STOP_HALLWAY[room_index] + step, end, step
                    ):
                        if hallway_index in NON_STOP_HALLWAY:
                            continue
                        if (
                            state.hallway[hallway_index] is not None
                        ):  # blocked by other amphipod
                            break  # change direction
                        new_state = State(
                            state.energy
                            + (
                                abs(hallway_index - NON_STOP_HALLWAY[room_index])
                                + room_size
                                - len(room)
                                + 1
                            )
                            * (10**amphipod_type),
                            insert_tuple(
                                state.rooms, room_index, room[:-1]
                            ),  # remove top room
                            insert_tuple(
                                state.hallway, hallway_index, amphipod_type
                            ),  # add to hallway
                        )
                        if new_state.fingerprint not in visited:
                            heapq.heappush(pq, new_state)

        # move from hallyway to room
        for hallway_index, amphipod_type in enumerate(state.hallway):
            if amphipod_type is None:
                continue
            # blocked by other amphipod in hallway
            if hallway_index < NON_STOP_HALLWAY[amphipod_type] and any(
                state.hallway[hallway_index + 1 : NON_STOP_HALLWAY[amphipod_type]]
            ):
                continue
            if hallway_index > NON_STOP_HALLWAY[amphipod_type] and any(
                state.hallway[NON_STOP_HALLWAY[amphipod_type] + 1:hallway_index]
            ):
                continue
            home_room = state.rooms[amphipod_type]
            if home_room and any(r != amphipod_type for r in home_room):
                continue  # not same type in room
            new_state = State(
                state.energy
                + (
                    abs(hallway_index - NON_STOP_HALLWAY[amphipod_type])
                    + room_size
                    - len(home_room)
                )
                * (10**amphipod_type),
                insert_tuple(state.rooms, amphipod_type, home_room + (amphipod_type,)),
                insert_tuple(state.hallway, hallway_index, None),
            )
            if new_state.fingerprint not in visited:
                heapq.heappush(pq, new_state)

    return 0


class Test(unittest.TestCase):
    def test_part_one(self):
        start = State(0, ((0, 1), (3, 2), (2, 1), (0, 3)))
        self.assertEqual(solve(start), 12521)

    def test_part_two(self):
        start = State(0, ((0, 3, 3, 1), (3, 1, 2, 2), (2, 0, 1, 1), (0, 2, 0, 3)))
        self.assertEqual(solve(start), 44169)


# A: 0, B: 1, C: 2, D: 3
if __name__ == "__main__":
    part1_start = State(0, ((3, 3), (2, 2), (1, 0), (0, 1)))
    print(solve(part1_start))
    part2_start = State(0, ((3, 3, 3, 3), (2, 1, 2, 2), (1, 0, 1, 0), (0, 2, 0, 1)))
    print(solve(part2_start))
