use clap::{arg, command, Parser};
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Part of the problem we want to solve 1 or 2, 1 by default
    #[arg(short, long, default_value_t = 1)]
    part: u8,

    /// Path to the file with the input
    #[arg(short, long)]
    input: String,
}
fn to_numbers(input: String) -> String {
    // The reasons I'm adding the character version of the numbers to the replacement is
    // to support one character being used by two numbers eg. 4nineightseven2 -> 49872
    // (and also because I'm quite bad with regex)
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "siz6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}
fn main() {
    // --snip--
    let args = Args::parse();
    println!("In file {}", args.input);

    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut total = 0;
    for line in lines {
        let mut line_string = line.to_string();
        if args.part == 2 {
            line_string = to_numbers(line_string);
        }

        let numbers: String = line_string.chars().filter(|c| c.is_numeric()).collect();
        let two_digits_number_str = format!(
            "{}{}",
            numbers.chars().nth(0).unwrap().to_string(),
            numbers.chars().nth(numbers.len() - 1).unwrap().to_string()
        );
        let number = two_digits_number_str.parse::<i32>().unwrap();
        total = total + number;
    }

    print!("{total}");
}
