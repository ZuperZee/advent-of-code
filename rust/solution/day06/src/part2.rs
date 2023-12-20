pub fn parse(input: &str) -> u64 {
    let lines_numbers = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1) // Skip the descriptor name (Time: and Distance:)
                .collect::<Vec<_>>()
                .join("")
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<u64>>();

    let time_numbers = lines_numbers[0];
    let distance_numbers = lines_numbers[1];

    ways_to_beat_records_count(time_numbers, distance_numbers)
}

fn ways_to_beat_records_count(time: u64, distance: u64) -> u64 {
    let time = time as f64;
    let distance = distance as f64;

    let x = (time.powf(2.0) - 4.0 * distance).sqrt();

    let min = (time - x) / 2.0 + 1.0;
    let max = (time + x) / 2.0 - 1.0;

    (max.ceil() - min.floor() + 1.0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = std::fs::read_to_string("../../../input/day06_example.txt")
            .expect("example file should exist");
        let result = parse(&example_input);
        assert_eq!(result, 71503);
    }
}
