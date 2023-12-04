pub fn parse(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_game_id, game_sets) = line.split_once(':').unwrap();

            let mut game = Game::default();

            for game_set in game_sets.split(';') {
                for cubes in game_set.split(',') {
                    let (cube_amount, cube_color) = cubes.trim().split_once(' ').unwrap();
                    let cube_amount = cube_amount.parse::<u32>().unwrap();
                    game.checked_set_cube(cube_color, cube_amount);
                }
            }

            game.get_power()
        })
        .sum()
}

#[derive(Default)]
struct Game {
    red_amount: u32,
    green_amount: u32,
    blue_amount: u32,
}

impl Game {
    /// Only adds if it's more than existing value
    fn checked_set_cube(&mut self, cube_color: &str, cube_amount: u32) {
        let current_cube_amount = match cube_color {
            "red" => &mut self.red_amount,
            "green" => &mut self.green_amount,
            "blue" => &mut self.blue_amount,
            _ => unreachable!(),
        };

        if cube_amount > *current_cube_amount {
            *current_cube_amount = cube_amount
        }
    }

    fn get_power(&self) -> u32 {
        self.red_amount * self.green_amount * self.blue_amount
    }
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
        assert_eq!(result, 2286);
    }
}
