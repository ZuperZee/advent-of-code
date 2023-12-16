use std::time::SystemTime;

use day01::*;

fn main() {
    let input = std::fs::read_to_string("../input/day01.txt").expect("input file should exist");

    let now = SystemTime::now();
    let result = part1::parse(&input);
    let elapsed = now.elapsed().unwrap();
    println!("Part 1 result: {}", result);
    println!("Executed in {:?}", elapsed);

    let now = SystemTime::now();
    let result = part2::parse(&input);
    let elapsed = now.elapsed().unwrap();
    println!("Part 2 result: {}", result);
    println!("Executed in {:?}", elapsed);
}
