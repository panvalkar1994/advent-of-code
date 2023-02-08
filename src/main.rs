use std::fs;

use advent_of_code::aoc15::day1;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let input = fs::read_to_string(file).unwrap();

    let solution = day1::get_basement(input);
    println!("Solution: {solution}");
}
