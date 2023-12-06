# Advent of Code 2023
This project will contain my solutions as I work through the [2023 Advent of Code](https://adventofcode.com/2023).

In the process I will be teaching myself the Rust programming language, so the quality
of the solutions will definitely vary from day to day.

The solutions for each day will reside in its own test. The samples provided in the puzzle will be used to create
unit tests, while the main executable will read in the input that was provided to me from the appropriate file.
The main executable will default to running code to solve the latest posted puzzle that I have solved, but may
be used to solve a puzzle from a different day by passing that day as a parameter on the command line.
For example, to run the solution for the puzzle from the 5th day, use `cargo run -- 5`.
