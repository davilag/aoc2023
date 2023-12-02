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

struct Cubes {
    blue: i32,
    green: i32,
    red: i32,
}

impl Cubes {
    fn new(game: String) -> Cubes {
        let cubes = game.split(",");
        let mut game_map: HashMap<&str, i32> = HashMap::new();
        for cube in cubes {
            let mut input = cube.trim().split(" ");
            let number = input.next().unwrap().parse::<i32>().unwrap();
            let colour = input.next().unwrap();
            game_map.insert(colour, number);
        }

        Cubes {
            blue: *game_map.get("blue").unwrap_or(&0),
            green: *game_map.get("green").unwrap_or(&0),
            red: *game_map.get("red").unwrap_or(&0),
        }
    }

    fn is_possible(&self, max_blue: i32, max_green: i32, max_red: i32) -> bool {
        return self.blue <= max_blue && self.green <= max_green && self.red <= max_red;
    }
}

struct Game {
    id: i32,
    withdraws: Vec<Cubes>,
}

impl Game {
    fn new(line: String) -> Game {
        let mut id_and_withdraws = line.split(":");
        let id = id_and_withdraws
            .next()
            .unwrap()
            .to_string()
            .strip_prefix("Game ")
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let withdraws_strs = id_and_withdraws.next().unwrap().split(";");
        let mut withdraws: Vec<Cubes> = vec![];
        for withdraw_str in withdraws_strs {
            withdraws.push(Cubes::new(withdraw_str.to_string()));
        }

        Game {
            id: id,
            withdraws: withdraws,
        }
    }

    fn is_possible(&self, max_blue: i32, max_green: i32, max_red: i32) -> bool {
        for withdraw in self.withdraws.iter() {
            if !withdraw.is_possible(max_blue, max_green, max_red) {
                return false;
            }
        }

        true
    }

    fn min_cubes(&self) -> Cubes {
        let mut out = Cubes {
            blue: 0,
            green: 0,
            red: 0,
        };
        for withdraw in self.withdraws.iter() {
            if withdraw.blue > out.blue {
                out.blue = withdraw.blue;
            }
            if withdraw.green > out.green {
                out.green = withdraw.green;
            }
            if withdraw.red > out.red {
                out.red = withdraw.red;
            }
        }
        out
    }
}

fn main() {
    let args = Args::parse();
    let file_content =
        fs::read_to_string(args.input).expect("Should have been able to read the file");
    let lines = file_content.lines();
    let mut total = 0;
    for line in lines {
        let game = Game::new(line.to_string());
        if args.part == 2 {
            total += game.min_cubes().red * game.min_cubes().green * game.min_cubes().blue;
        } else {
            if !game.is_possible(14, 13, 12) {
                continue;
            }
            total += game.id;
        }
    }

    println!("{total}");
}
