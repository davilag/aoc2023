use clap::{arg, command, Parser};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::Lines;

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

struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    numbers: HashSet<i32>,
}
fn get_card_id(card_str: String) -> i32 {
    let id_str = card_str.strip_prefix("Card").unwrap().trim();
    id_str.to_string().parse::<i32>().unwrap()
}

fn get_numbers(numbers_str: String) -> HashSet<i32> {
    let numbers_strs = numbers_str.trim().split_whitespace();
    let mut out: HashSet<i32> = HashSet::new();
    for number_str in numbers_strs {
        out.insert(number_str.to_string().trim().parse::<i32>().unwrap());
    }

    out
}

impl Card {
    fn new(line: String) -> Card {
        let mut card_and_numbers = line.split(":");
        let card_id = get_card_id(card_and_numbers.next().unwrap().to_string());
        let both_cards = card_and_numbers.next().unwrap().to_string();
        let mut cards = both_cards.split("|");

        Card {
            id: card_id,
            winning_numbers: get_numbers(cards.next().unwrap().to_string()),
            numbers: get_numbers(cards.next().unwrap().to_string()),
        }
    }

    fn points(&self) -> i32 {
        let multiplier = 2;
        let mut out = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(&number) {
                if out == 0 {
                    out = 1
                } else {
                    out *= multiplier;
                }
            }
        }

        out
    }

    fn match_numbers(&self) -> i32 {
        let mut out = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(&number) {
                out += 1;
            }
        }

        out
    }
}

fn part_one(lines: Lines) -> i32 {
    let mut out = 0;
    for line in lines {
        let card = Card::new(line.to_string());
        out += card.points();
    }

    out
}

fn part_two(lines: Lines) -> i32 {
    let mut out = 0;
    let mut card_map: HashMap<usize, Card> = HashMap::new();
    let mut n_lines = 0;
    let mut occurrences_map: HashMap<usize, i32> = HashMap::new();
    for line in lines {
        let card = Card::new(line.to_string());
        card_map.insert(card.id as usize, card);
        n_lines += 1;
    }

    for n in 1..(n_lines + 1) {
        let card = card_map.get(&n).unwrap();
        let n_occurrences = *occurrences_map.get(&n).unwrap_or(&1);
        out += n_occurrences;
        let card_match = card.match_numbers() as usize;
        for o in n + 1..(n + card_match + 1) {
            if o > n_lines {
                continue;
            }
            let current_occurrences = occurrences_map.get(&o).unwrap_or(&1);
            occurrences_map.insert(o, current_occurrences + n_occurrences);
        }
    }
    out
}

fn main() {
    let args = Args::parse();
    println!("In file {}", args.input);

    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let lines = contents.lines();

    if args.part == 1 {
        println!("{}", part_one(lines));
    } else {
        println!("{}", part_two(lines));
    }
}
