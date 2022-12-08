import itertools
import sys
from collections import namedtuple

WorkRange = namedtuple("WorkRange", ["start", "end"])


def main() -> None:
    overlap_pairs = 0

    with open("day4_input") as f:
        for line in f:
            range1, range2 = line.strip().split(",")
            work_range1 = WorkRange(*[int(v) for v in range1.split("-")])
            work_range2 = WorkRange(*[int(v) for v in range2.split("-")])

            if (
                (
                    work_range1.start >= work_range2.start
                    and work_range1.start <= work_range2.end
                )
                or (
                    work_range1.end >= work_range2.start
                    and work_range1.end <= work_range2.end
                )
                or (
                    work_range2.start >= work_range1.start
                    and work_range2.start <= work_range1.end
                )
                or (
                    work_range2.end >= work_range1.start
                    and work_range2.end <= work_range1.end
                )
            ):
                overlap_pairs += 1

    print(overlap_pairs)


if __name__ == "__main__":
    main()
