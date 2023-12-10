def parse(input: str) -> int:
    total_scratchcard_amount = 0
    total_copies: list[int] = []

    for line in input.splitlines():
        scratchcards = 1
        if len(total_copies) > 0:
            scratchcards += total_copies.pop(0)

        total_scratchcard_amount += scratchcards

        times_won = 0
        [_, game] = line.split(":")
        [winning_numbers_str, own_numbers_str] = game.split("|")

        winning_numbers = string_numbers_to_numbers(winning_numbers_str)
        own_numbers = string_numbers_to_numbers(own_numbers_str)

        for own_number in own_numbers:
            won = own_number in winning_numbers
            if won:
                times_won += 1

        for i in range(times_won):
            if len(total_copies) >= i + 1:
                total_copies[i] += scratchcards
            else:
                total_copies.append(scratchcards)

    return total_scratchcard_amount


def string_numbers_to_numbers(string: str) -> list[int]:
    numbers = []
    for number in string.split(" "):
        if number != "":
            numbers.append(int(number))
    return numbers
