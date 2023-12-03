const VALID_DIGIT_WORDS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn parse_digit(line: &[u8]) -> Option<u32> {
    let letter = &line[0];
    if matches!(letter, b'1'..=b'9') {
        return Some(letter.wrapping_sub(b'0').into());
    }

    VALID_DIGIT_WORDS
        .into_iter()
        .enumerate()
        .find(|(_digit_index, digit_word)| {
            line.len() >= digit_word.len() && *digit_word == &line[0..digit_word.len()]
        })
        .map(|(digit_index, _digit_word)| digit_index as u32 + 1)
}

fn get_digit(line: &[u8]) -> u32 {
    (0..line.len())
        .find_map(|letter_index| parse_digit(&line[letter_index..]))
        .unwrap()
}

fn get_digit_rev(line: &[u8]) -> u32 {
    (0..line.len())
        .rev()
        .find_map(|letter_index| parse_digit(&line[letter_index..]))
        .unwrap()
}

pub fn parse(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = get_digit(line.as_bytes());
            let last_digit = get_digit_rev(line.as_bytes());
            first_digit * 10 + last_digit
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = parse(example_input);
        assert_eq!(result, 281);
    }
}
