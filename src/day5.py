import sys


def main() -> None:
    stacks = [[], [], [], [], [], [], [], [], [], []]
    parsing_stacks = True

    with open("day5_input") as f:
        for line in f:
            line = line.removesuffix("\n")
            if not line:
                continue
            if parsing_stacks:
                if "[" in line:
                    for i in range(1, 35, 4):
                        if line[i] != " ":
                            stacks[(i - 1) // 4].append(line[i])
                else:
                    parsing_stacks = False
                    for stack in stacks:
                        stack.reverse()
            else:
                _, move_count, _, source, _, dest = line.split()
                items = []
                for _ in range(int(move_count)):
                    items.append(stacks[int(source) - 1].pop())

                items.reverse()
                for item in items:
                    stacks[int(dest) - 1].append(item)

        top_items = ""
        for stack in stacks:
            if stack:
                top_items += stack.pop()

        print(top_items)


if __name__ == "__main__":
    main()
