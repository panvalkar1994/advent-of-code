use std::fs;

use advent_of_code::aoc15::day6;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let input = fs::read_to_string(file).unwrap();

    let solution = day6::get_all_lite_lights(input);
    println!("Solution: {solution}");
}
