use std::fs;

use advent_of_code::aoc15::day7;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let input = fs::read_to_string(file).unwrap();

    let solution = day7::get_circuit_input(input, "a");
    println!("Solution: {solution}");


}
