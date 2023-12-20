pub fn parse(input: &str) -> u32 {
    let lines_numbers = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1) // Skip the descriptor name (Time: and Distance:)
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let time_numbers = &lines_numbers[0];
    let distance_numbers = &lines_numbers[1];

    time_numbers
        .iter()
        .zip(distance_numbers.iter())
        .map(|(t, d)| ways_to_beat_records_count(*t, *d))
        .product::<u32>()
}

fn ways_to_beat_records_count(time: u32, distance: u32) -> u32 {
    let time = time as f64;
    let distance = distance as f64;

    let x = (time.powf(2.0) - 4.0 * distance).sqrt();

    let min = (time - x) / 2.0 + 1.0;
    let max = (time + x) / 2.0 - 1.0;

    (max.ceil() - min.floor() + 1.0) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = std::fs::read_to_string("../../../input/day06_example.txt")
            .expect("example file should exist");
        let result = parse(&example_input);
        assert_eq!(result, 288);
    }
}
