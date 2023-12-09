pub fn parse(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let grid: Grid = grid.as_slice();

    let all_grid_numbers = get_all_grid_numbers(grid);

    all_grid_numbers
        .iter()
        .filter(|grid_number| {
            grid.iter()
                .take(grid_number.row_nr + 2)
                .skip(grid_number.row_nr.saturating_sub(1))
                .any(|row| {
                    row.iter()
                        .take(grid_number.column_nr_end + 2)
                        .skip(grid_number.column_nr_start.saturating_sub(1))
                        .any(|cell| *cell != b'.' && !cell.is_ascii_digit())
                })
        })
        .map(|grid_number| grid_number.number)
        .sum()
}

fn get_all_grid_numbers(grid: Grid) -> Vec<GridNumber> {
    let mut grid_numbers: Vec<GridNumber> = vec![];
    let mut grid_number = GridNumber::default();

    for (row_nr, row) in grid.iter().enumerate() {
        for (column_nr, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                let digit = (cell - b'0') as u32;
                if grid_number.number == 0 {
                    grid_number.number = digit;
                    grid_number.column_nr_start = column_nr;
                    grid_number.column_nr_end = column_nr;
                    grid_number.row_nr = row_nr;
                } else {
                    grid_number.number = grid_number.number * 10 + digit;
                    grid_number.column_nr_end += 1;
                }
            } else if grid_number.number != 0 {
                grid_numbers.push(grid_number.clone());
                grid_number.number = 0;
            }
        }
    }

    grid_numbers
}

type Grid<'a> = &'a [&'a [u8]];

#[derive(Default, Clone)]
struct GridNumber {
    number: u32,
    row_nr: usize,
    column_nr_start: usize,
    column_nr_end: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example() {
        let example_input = "\
467..114..
...*...111
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = parse(example_input);
        assert_eq!(result, 4361);
    }
}
