import sys


def main() -> None:
    input_file_path = sys.argv[1]
    total_score = 0
    with open(input_file_path) as f:
        for line in f:
            opponent_choice, my_choice = line.split()

            match opponent_choice:
                case "A":
                    choice_score = 1
                case "B":
                    choice_score = 2
                case "C":
                    choice_score = 3

            match my_choice:
                case "X":
                    choice_score -= 1
                    if choice_score == 0:
                        choice_score = 3
                case "Y":
                    total_score += 3  # draw
                case "Z":
                    choice_score += 1
                    if choice_score == 4:
                        choice_score = 1
                    total_score += 6  # win

            total_score += choice_score

    print(total_score)


if __name__ == "__main__":
    main()
