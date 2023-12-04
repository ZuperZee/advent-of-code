pub fn parse(input: &str) -> u32 {
    let color_maxes: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    input
        .lines()
        .filter_map(|line| {
            let (game_id, game_sets) = line.split_once(':').unwrap();

            let is_invalid_game = game_sets.split(';').any(|game_set| {
                game_set.split(',').any(|cubes| {
                    let (cube_amount, cube_color) = cubes.trim().split_once(' ').unwrap();
                    let cube_amount = cube_amount.parse::<u32>().unwrap();
                    let max_cube_amount = color_maxes
                        .iter()
                        .find_map(|(color, amount)| {
                            if *color == cube_color {
                                Some(amount)
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    cube_amount > *max_cube_amount
                })
            });

            if is_invalid_game {
                return None;
            }

            let (_, game_id) = game_id.split_once(' ').unwrap();
            let game_id = game_id.parse::<u32>().unwrap();
            Some(game_id)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = parse(example_input);
        assert_eq!(result, 8);
    }
}
