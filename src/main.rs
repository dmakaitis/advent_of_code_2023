mod day01;
mod day02;
mod day03;

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle_number = if args.len() >= 2 { args[1].parse().ok() } else { None };

    let output1: String;
    let output2: String;

    match puzzle_number {
        Some(1) => {
            let input = read_input_file("input01.txt");
            output1 = day01::part_one(&input).to_string();
            output2 = day01::part_two(&input).to_string();
        },
        Some(2) => {
            let input = read_input_file("input02.txt");
            output1 = day02::part_one(&input).to_string();
            output2 = day02::part_two(&input).to_string();
        },
        _ => {
            let input = read_input_file("input03.txt");
            output1 = day03::part_one(&input).to_string();
            output2 = day03::part_two(&input).to_string();
        }
    }

    println!("Part one output: {output1}");
    println!("Part two output: {output2}");
}

fn read_input_file(filename: &str) -> String {
    let mut input = read_to_string(filename).unwrap();
    input.retain(|c| c != '\r');
    input
}
