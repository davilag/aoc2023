use clap::{arg, command, Parser};
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

struct MapLine {
    destination_start: i64,
    origin_start: i64,
    range: i64,
}

struct SeedRange {
    start: i64,
    size: i64,
}

impl SeedRange {
    fn end(&self) -> i64 {
        self.start + self.size
    }
    fn matches_range(&self, other: &SeedRange) -> bool {
        if self.start <= other.start {
            return self.end() >= other.start;
        } else {
            return other.end() >= self.start;
        }
    }

    fn get_overlap(&self, other: &SeedRange) -> Option<SeedRange> {
        if !self.matches_range(other) {
            println!("they don't overlap!");
            return None;
        }

        if self.start <= other.start {
            if self.end() >= other.end() {
                return Some(SeedRange {
                    start: other.start,
                    size: other.size,
                });
            } else {
                return Some(SeedRange {
                    start: other.start,
                    size: self.size - other.start,
                });
            }
        } else {
            if self.end() <= other.end() {
                return Some(SeedRange {
                    start: self.start,
                    size: self.size,
                });
            }
            return Some(SeedRange {
                start: self.start,
                size: self.start - other.end(),
            });
        }
    }
}

impl MapLine {
    fn new(line: &str) -> MapLine {
        let mut numbers_strs = line.trim().split_whitespace();
        let destination_range_start = numbers_strs.next().unwrap().parse::<i64>().unwrap();
        let source_range_start = numbers_strs.next().unwrap().parse::<i64>().unwrap();
        let range_length = numbers_strs.next().unwrap().parse::<i64>().unwrap();

        MapLine {
            destination_start: destination_range_start,
            origin_start: source_range_start,
            range: range_length,
        }
    }

    fn is_in_range(&self, number: i64) -> bool {
        number >= self.origin_start && number < self.origin_start + self.range
    }

    fn calculate_destination(&self, origin: i64) -> i64 {
        if self.is_in_range(origin) {
            return origin - self.origin_start + self.destination_start;
        }

        origin
    }

    fn is_optimal(&self) -> bool {
        self.destination_start < self.origin_start
    }

    fn optimal_range(&self) -> SeedRange {
        if self.is_optimal() {
            return SeedRange {
                start: self.origin_start,
                size: self.range,
            };
        }

        SeedRange {
            start: 0,
            size: self.origin_start,
        }
    }
}

struct Map {
    origin: String,
    destination: String,
    lines: Vec<MapLine>,
}

impl Map {
    fn new(line: &str) -> Map {
        let mut origin_and_dest = line.trim().strip_suffix(" map:").unwrap().split("-");
        let origin = origin_and_dest.next().unwrap().to_string();
        origin_and_dest.next();
        let destination = origin_and_dest.next().unwrap().to_string();

        Map {
            origin: origin,
            destination: destination,
            lines: vec![],
        }
    }

    fn get_destination(&self, origin: i64) -> i64 {
        for line in &self.lines {
            if line.is_in_range(origin) {
                return line.calculate_destination(origin);
            }
        }

        origin
    }

    fn get_optimal_path(&self, seed_range: SeedRange) -> SeedRange {
        let mut optimal_range = seed_range;
        for i in 0..self.lines.len() {
            let line_optimal_range = self.lines[i].optimal_range();
            println!(
                "Line optimal range for {} {} {} is: {} {}",
                self.lines[i].destination_start,
                self.lines[i].origin_start,
                self.lines[i].range,
                line_optimal_range.start,
                line_optimal_range.size,
            );
            optimal_range = line_optimal_range
                .get_overlap(&optimal_range)
                .unwrap_or(optimal_range);
            println!(
                "optimal range now is: {} {}",
                optimal_range.start, optimal_range.size
            );
        }
        optimal_range
    }
}

fn get_seeds(seeds_str: String) -> Vec<i64> {
    let mut numbers_strs = seeds_str
        .trim()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace();
    let mut out: Vec<i64> = Vec::new();
    for number_str in numbers_strs {
        out.push(number_str.to_string().trim().parse::<i64>().unwrap())
    }

    out
}

fn is_starting_map_line(line: &str) -> bool {
    line.to_string().contains("map:")
}

fn part_one(mut lines: Lines) -> i64 {
    let seeds = get_seeds(lines.next().unwrap().to_string());
    println!("{:?}", seeds);
    let mut maps: Vec<Map> = vec![];
    let mut current_map: Option<Map> = None;
    for line in lines {
        if line == "" {
            match current_map {
                Some(map) => maps.push(map),
                None => {}
            }
            current_map = None;
            continue;
        }

        if is_starting_map_line(line) {
            current_map = Some(Map::new(line));
            continue;
        }

        let map_line = MapLine::new(line);
        current_map.as_mut().unwrap().lines.push(map_line);
    }
    match current_map {
        Some(map) => maps.push(map),
        None => {}
    }
    let mut out = 0;
    for i in 0..seeds.len() {
        let mut location = seeds[i];
        for j in 0..maps.len() {
            location = maps[j].get_destination(location);
        }
        if out == 0 || location <= out {
            out = location;
        }
    }
    out
}

fn get_seed_ranges(line: String) -> Vec<SeedRange> {
    let mut numbers_strs = line
        .trim()
        .strip_prefix("seeds:")
        .unwrap()
        .split_whitespace();
    let mut out: Vec<SeedRange> = Vec::new();
    while numbers_strs.clone().count() > 0 {
        out.push(SeedRange {
            start: numbers_strs.next().unwrap().parse::<i64>().unwrap(),
            size: numbers_strs.next().unwrap().parse::<i64>().unwrap(),
        })
    }

    out
}

fn part_two(mut lines: Lines) -> i64 {
    let seed_ranges = get_seed_ranges(lines.next().unwrap().to_string());
    let mut maps: Vec<Map> = vec![];
    let mut current_map: Option<Map> = None;
    for line in lines {
        if line == "" {
            match current_map {
                Some(map) => maps.push(map),
                None => {}
            }
            current_map = None;
            continue;
        }

        if is_starting_map_line(line) {
            current_map = Some(Map::new(line));
            continue;
        }

        let map_line = MapLine::new(line);
        current_map.as_mut().unwrap().lines.push(map_line);
    }
    match current_map {
        Some(map) => maps.push(map),
        None => {}
    }
    let mut out = 0;
    for seed_range in seed_ranges {
        for i in seed_range.start..seed_range.end() {
            let mut location = i;
            for j in 0..maps.len() {
                location = maps[j].get_destination(location);
            }
            if out == 0 || location <= out {
                out = location;
            }
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
