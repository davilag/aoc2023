use clap::{arg, command, Parser};
use std::fs;

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

struct Race {
    time: i64,
    record_distance: i64,
}

fn calculate_distance(speed: i64, time: i64) -> i64 {
    speed * time
}

impl Race {
    fn new(time: i64, record_distance: i64) -> Race {
        Race {
            time,
            record_distance,
        }
    }

    fn spare_time(&self, speed: i64) -> i64 {
        self.time - speed
    }
}

fn get_input(part: u8, line: &str, prefix: &str) -> Vec<i64> {
    let mut clean_line = line.strip_prefix(prefix).unwrap().to_string();
    if part == 2 {
        clean_line = clean_line.replace(" ", "");
    }

    let numbers_strs = clean_line.split_whitespace().into_iter();
    let mut out: Vec<i64> = vec![];
    for number_str in numbers_strs {
        out.push(number_str.to_string().parse().unwrap())
    }

    out
}

fn main() {
    let args = Args::parse();
    println!("In file {}", args.input);

    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let times = get_input(args.part, lines.next().unwrap(), "Time: ");
    let distances = get_input(args.part, lines.next().unwrap(), "Distance:");

    if times.len() != distances.len() {
        panic!("Times and distances should have the same length");
    }

    let mut out = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        let race = Race::new(*time, *distance);
        let mut possibilities = 0;
        for i in 0..=race.time {
            let distance = calculate_distance(i, race.spare_time(i));
            if distance > race.record_distance {
                possibilities = possibilities + 1;
            }
        }
        println!("possibilities: {possibilities}");
        out = out * possibilities;
    }

    println!("{out}");
}
