extern crate core;

use num_format::{Locale, ToFormattedString};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::env;
use std::fs::read_to_string;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let puzzle_number = if args.len() >= 2 {
        args[1].parse().ok()
    } else {
        None
    };

    let output1: String;
    let output2: String;

    let start = SystemTime::now();

    match puzzle_number {
        Some(1) => {
            let input = read_input_file("input/day01.txt");
            output1 = day01::part_one(&input).to_string();
            output2 = day01::part_two(&input).to_string();
        }
        Some(2) => {
            let input = read_input_file("input/day02.txt");
            output1 = day02::part_one(&input).to_string();
            output2 = day02::part_two(&input).to_string();
        }
        Some(3) => {
            let input = read_input_file("input/day03.txt");
            output1 = day03::part_one(&input).to_string();
            output2 = day03::part_two(&input).to_string();
        }
        Some(4) => {
            let input = read_input_file("input/day04.txt");
            output1 = day04::part_one(&input).to_string();
            output2 = day04::part_two(&input).to_string();
        }
        Some(5) => {
            let input = read_input_file("input/day05.txt");
            output1 = day05::part_one(&input).to_string();
            output2 = day05::part_two(&input).to_string();
        }
        Some(6) => {
            let input = read_input_file("input/day06.txt");
            output1 = day06::part_one(&input).to_string();
            output2 = day06::part_two(&input).to_string();
        }
        Some(7) => {
            let input = read_input_file("input/day07.txt");
            output1 = day07::part_one(&input).to_string();
            output2 = day07::part_two(&input).to_string();
        }
        Some(8) => {
            let input = read_input_file("input/day08.txt");
            output1 = day08::part_one(&input).to_string();
            output2 = day08::part_two(&input).to_string();
        }
        Some(9) => {
            let input = read_input_file("input/day09.txt");
            output1 = day09::part_one(&input).to_string();
            output2 = day09::part_two(&input).to_string();
        }
        Some(10) => {
            let input = read_input_file("input/day10.txt");
            output1 = day10::part_one(&input).to_string();
            output2 = day10::part_two(&input).to_string();
        }
        Some(11) => {
            let input = read_input_file("input/day11.txt");
            output1 = day11::part_one(&input).to_string();
            output2 = day11::part_two(&input).to_string();
        }
        Some(12) => {
            let input = read_input_file("input/day12.txt");
            output1 = day12::part_one(&input).to_string();
            output2 = day12::part_two(&input).to_string();
        }
        Some(13) => {
            let input = read_input_file("input/day13.txt");
            output1 = day13::part_one(&input).to_string();
            output2 = day13::part_two(&input).to_string();
        }
        Some(14) => {
            let input = read_input_file("input/day14.txt");
            output1 = day14::part_one(&input).to_string();
            output2 = day14::part_two(&input).to_string();
        }
        Some(15) => {
            let input = read_input_file("input/day15.txt");
            output1 = day15::part_one(&input).to_string();
            output2 = day15::part_two(&input).to_string();
        }
        Some(16) => {
            let input = read_input_file("input/day16.txt");
            output1 = day16::part_one(&input).to_string();
            output2 = day16::part_two(&input).to_string();
        }
        Some(17) => {
            let input = read_input_file("input/day17.txt");
            output1 = day17::part_one(&input).to_string();
            output2 = day17::part_two(&input).to_string();
        }
        Some(18) => {
            let input = read_input_file("input/day18.txt");
            output1 = day18::part_one(&input).to_string();
            output2 = day18::part_two(&input).to_string();
        }
        Some(19) => {
            let input = read_input_file("input/day19.txt");
            output1 = day19::part_one(&input).to_string();
            output2 = day19::part_two(&input).to_string();
        }
        Some(20) => {
            let input = read_input_file("input/day20.txt");
            output1 = day20::part_one(&input).to_string();
            output2 = day20::part_two(&input).to_string();
        }
        Some(21) => {
            let input = read_input_file("input/day21.txt");
            output1 = day21::part_one(&input).to_string();
            output2 = day21::part_two(&input).to_string();
        }
        Some(22) => {
            let input = read_input_file("input/day22.txt");
            output1 = day22::part_one(&input).to_string();
            output2 = day22::part_two(&input).to_string();
        }
        Some(23) => {
            let input = read_input_file("input/day23.txt");
            output1 = day23::part_one(&input).to_string();
            output2 = day23::part_two(&input).to_string();
        }
        Some(24) => {
            let input = read_input_file("input/day24.txt");
            output1 = day24::part_one(&input).to_string();
            output2 = day24::part_two(&input).to_string();
        }
        _ => {
            let input = read_input_file("input/day25.txt");
            output1 = day25::part_one(&input).to_string();
            output2 = day25::part_two(&input).to_string();
        }
    }

    let elapsed = start.elapsed();

    println!("Part one output: {output1}");
    println!("Part two output: {output2}");

    if let Ok(time) = elapsed {
        println!(
            "Total elapsed time: {} ns",
            time.as_nanos().to_formatted_string(&Locale::en)
        );
    }
}

/// Returns the contents of the given file, with any '\r' characters stripped out so we don't
/// have to worry about them when running on a Windows system.
///
/// # Arguments
///
/// * `filename` - The name of the file to read
fn read_input_file(filename: &str) -> String {
    let mut input = read_to_string(filename).unwrap();
    input.retain(|c| c != '\r');
    input
}
