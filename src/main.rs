use std::fs;

use advent_of_code::aoc15::day3;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = args.get(1).unwrap();
    let input = fs::read_to_string(file).unwrap();

    let solution = day3::get_count_atleast_once_visited_home_with_robo(input);
    println!("Solution: {solution}");
}
