use regex::Regex;
use std::error::Error;
use std::fs;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Game {
    player_count: i32,
    last_marble_worth: i32,
    high_score: i32,
    circle: Vec<i32>,
    player_score: Vec<i32>,
}

impl Game {
    fn new(player_count: i32, last_marble_worth: i32, high_score: i32) -> Self {
        Game {
            player_count,
            last_marble_worth,
            high_score,
            // init with 0 marble
            circle: vec![0; 1],
            player_score: vec![],
        }
    }
}

fn main() -> Result<()> {
    // Extract all numbers
    let re = Regex::new(r"\d+").unwrap();
    let input = fs::read_to_string("input/test.txt")?;

    let numbers: Vec<i32> = re
        // iterates over all matches
        .find_iter(&input)
        // filters and maps, filter only yield those for which the supplied closure provides
        // returns Some(Value)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();

    if numbers.len() != 3 {
        return err!("error parsing input");
    }

    let game: Game = Game::new(numbers[0], numbers[1], numbers[2]);

    part_1(&game)?;

    Ok(())
}

fn part_1(game: &Game) -> Result<()> {
    todo!()
}

fn part_2() {
    todo!()
}
