use std::fs;

use advent_of_code::aoc15::day2;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let input = fs::read_to_string(file).unwrap();

    let solution = day2::get_ribbon_length(input);
    println!("Solution: {solution}");
}
