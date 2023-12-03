use std::cmp::max;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Revelation {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Revelation {
    fn default() -> Revelation {
        return Revelation {
            red: 0,
            green: 0,
            blue: 0,
        };
    }
}

struct Game {
    // For the first part, we need an id here.
    revelations: Vec<Revelation>,
}

fn parse_revelation(s: String) -> Revelation {
    let mut revelation: Revelation = Revelation::default();

    s.split(',')
        .for_each(|s| match s.split(' ').collect::<Vec<&str>>().as_slice() {
            [_, quantity_str, color_str] => {
                if let Ok(quantity) = quantity_str.parse::<u32>() {
                    match color_str.as_ref() {
                        "red" => {
                            revelation.red += quantity;
                        }
                        "green" => {
                            revelation.green += quantity;
                        }
                        "blue" => {
                            revelation.blue += quantity;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });

    revelation
}

fn parse_game(s: String) -> Option<Game> {
    match s.split(':').collect::<Vec<&str>>().as_slice() {
        [_, revelation_str] => {
            let revelations: Vec<Revelation> = revelation_str
                .split(';')
                .map(|revelation_str| parse_revelation(revelation_str.to_string()))
                .collect::<Vec<Revelation>>();

            return Some(Game { revelations });
        }
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(input_filename) = args.get(1) {
        let input_file_reader = BufReader::new(File::open(input_filename)?);
        let mut sum_power = 0;

        for (line_index, line) in input_file_reader
            .lines()
            .map(|line| line.unwrap())
            .enumerate()
        {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            if let Some(Game { revelations }) = parse_game(line) {
                revelations.into_iter().for_each(|revelation| {
                    min_red = max(min_red, revelation.red);
                    min_green = max(min_green, revelation.green);
                    min_blue = max(min_blue, revelation.blue);
                });
            } else {
                eprintln!("Error: invalid input at line {}", line_index + 1);
            }

            sum_power += min_red * min_green * min_blue;
        }

        println!("{}", sum_power);
    } else {
        eprintln!("Usage: cargo run <input_filename>");
    }

    return Ok(());
}
