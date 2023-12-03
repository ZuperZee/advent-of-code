pub fn parse(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = line
                .bytes()
                .find(|b| matches!(b, b'1'..=b'9'))
                .unwrap()
                .wrapping_sub(b'0');
            let last_digit = line
                .bytes()
                .rfind(|b| matches!(b, b'1'..=b'9'))
                .unwrap()
                .wrapping_sub(b'0');

            (first_digit * 10 + last_digit) as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = parse(example_input);
        assert_eq!(result, 142);
    }

    #[test]
    fn parses_input() {
        let input = std::fs::read_to_string("../../../input/day01").expect("input file should exist");
        let result = parse(&input);
        assert_eq!(result, 55538);
    }
}
