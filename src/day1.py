import heapq
import sys


def main() -> None:
    input_file_path = sys.argv[1]
    max_calories = 0
    all_calories = []
    with open(input_file_path) as f:
        current_calories = 0
        for line in f:
            line = line.strip()
            if line:
                current_calories += int(line)
            else:
                if current_calories > max_calories:
                    max_calories = current_calories
                heapq.heappush(all_calories, current_calories)
                current_calories = 0

    print(max_calories)
    top_three = heapq.nlargest(3, all_calories)
    print(top_three)
    print(sum(top_three))


if __name__ == "__main__":
    main()
