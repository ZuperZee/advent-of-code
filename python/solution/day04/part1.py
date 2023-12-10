def parse(input: str) -> int:
    overall_points = 0

    for line in input.splitlines():
        times_won = 0
        [_, game] = line.split(":")
        [winning_numbers_str, own_numbers_str] = game.split("|")

        winning_numbers = string_numbers_to_numbers(winning_numbers_str)
        own_numbers = string_numbers_to_numbers(own_numbers_str)

        for own_number in own_numbers:
            won = own_number in winning_numbers
            if won:
                times_won += 1

        if times_won > 0:
            overall_points += 1 << times_won - 1

    return overall_points


def string_numbers_to_numbers(string: str) -> list[int]:
    numbers = []
    for number in string.split(" "):
        if number != "":
            numbers.append(int(number))
    return numbers
