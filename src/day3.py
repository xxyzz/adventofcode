import sys
import itertools


def main() -> None:
    total_priority = 0

    with open("day3_input") as f:
        for pack1, pack2, pack3 in itertools.zip_longest(*[f]* 3):
            badge = (set(pack1.strip()) & set(pack2.strip()) & set(pack3.strip())).pop()

            if badge.islower():
                total_priority += (ord(badge) - ord("a") + 1)
            else:
                total_priority += (ord(badge) - ord("A") + 27)

    print(total_priority)


if __name__ == "__main__":
    main()
