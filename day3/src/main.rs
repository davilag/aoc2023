use clap::{arg, command, Parser};
use std::collections::HashMap;
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

fn is_symbol(c: char) -> bool {
    return !c.is_numeric() && c != '.';
}

fn is_gear(c: char) -> bool {
    return c == '*';
}

fn get_touching_parts(parts: &Vec<Number>, row: i32, column: i32) -> Vec<i32> {
    let mut touching_parts: Vec<i32> = Vec::new();
    for part in parts {
        if part.is_adjacent(row, column) {
            touching_parts.push(part.calculate_value());
        }
    }
    return touching_parts;
}

struct Position {
    row: i32,
    start: i32,
    end: i32,
}
struct Number {
    position: Position,
    chars: Vec<char>,
}

impl Number {
    fn calculate_value(&self) -> i32 {
        let chars_to_string = self.chars.iter().collect::<String>();
        return chars_to_string.parse::<i32>().unwrap();
    }

    fn is_part(&self, map: &HashMap<i32, Vec<char>>) -> bool {
        for row_n in self.position.row - 1..(self.position.row + 2) {
            let row_option = map.get(&row_n);
            if row_option.is_none() {
                continue;
            }

            let row = row_option.unwrap();
            let mut start_index = self.position.start - 1;
            if start_index < 0 {
                start_index = 0;
            }
            let mut end_index = self.position.end + 1;
            if end_index > row.len() as i32 {
                end_index = row.len() as i32;
            }
            for n in start_index..end_index + 1 {
                if is_symbol(row[n as usize]) {
                    return true;
                }
            }
        }

        false
    }

    fn is_adjacent(&self, row: i32, column: i32) -> bool {
        return row >= self.position.row - 1
            && row <= self.position.row + 1
            && column >= self.position.start - 1
            && column <= self.position.end + 1;
    }
}

struct Schematic {
    numbers: Vec<Number>,
    map: HashMap<i32, Vec<char>>,
}

fn part_one(schematic: &Schematic) -> i32 {
    let mut total = 0;
    let numbers = &schematic.numbers;
    for number in numbers {
        if number.is_part(&schematic.map) {
            total = total + number.calculate_value();
        }
    }

    return total;
}

fn part_two(schematic: &Schematic) -> i32 {
    let mut total = 0;
    for (row_n, row) in schematic.map.iter() {
        for (column_n, char) in row.iter().enumerate() {
            if is_gear(*char) {
                let touching_parts =
                    get_touching_parts(&schematic.numbers, *row_n, column_n as i32);
                if touching_parts.len() == 2 {
                    total = total + touching_parts[0] * touching_parts[1];
                }
            }
        }
    }
    return total;
}

fn main() {
    let args = Args::parse();
    let file_content =
        fs::read_to_string(args.input).expect("Should have been able to read the file");
    let lines = file_content.lines();
    let mut schematic = Schematic {
        numbers: Vec::new(),
        map: HashMap::new(),
    };
    for (i, line) in lines.enumerate() {
        let line_string = line.to_string();
        let mut row: Vec<char> = vec![];
        let mut current_number: Option<Number> = None;
        for (j, char) in line_string.chars().enumerate() {
            row.push(char);
            if char.is_numeric() {
                if current_number.is_none() {
                    let number = Number {
                        chars: vec![char],
                        position: Position {
                            row: i as i32,
                            start: j as i32,
                            end: 0,
                        },
                    };
                    current_number = Some(number);
                } else {
                    let number = current_number.as_mut().unwrap();
                    number.chars.push(char);
                }
            } else {
                if current_number.is_some() {
                    let number = current_number.as_mut().unwrap();
                    number.position.end = (j - 1) as i32;

                    schematic.numbers.push(current_number.unwrap());
                    current_number = None;
                }
            }
        }

        schematic.map.insert(i as i32, row);
    }

    if args.part == 1 {
        println!("{}", part_one(&schematic));
    } else {
        println!("{}", part_two(&schematic));
    }
}
