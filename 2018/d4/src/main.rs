use std::error::Error;
use std::result;
use std::{collections::HashMap, fs};

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?;

    let mut lines: Vec<&str> = input.lines().collect();

    lines.sort_by_key(|line| extract_date(line).expect("Invalid format"));

    match part_1(&lines) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => println!("There was an error in Part 1: {}", err),
    };

    Ok(())
}

fn part_1(lines: &Vec<&str>) -> Result<usize> {
    let mut result: HashMap<usize, [usize; 60]> = HashMap::new();
    let mut sleep_start = 0usize;
    let mut id = 0usize;

    for line in lines {
        let (minute, action) = parse_line(line)?;

        match action {
            _ if action.starts_with("Guard ") => {
                let (_, id_content) = action
                    .split_once(" #")
                    .ok_or("Could not split Guard info")?;
                let (guard_id, _) = id_content.split_once(' ').ok_or("Could not split id")?;
                id = guard_id.parse::<usize>()?;
            }

            _ if action.starts_with("falls asleep") => {
                sleep_start = minute;
            }

            _ if action.starts_with("wakes up") => {
                let minute_sleep_count = result.entry(id).or_insert([0; 60]);

                if minute < sleep_start {
                    (sleep_start..60).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    });

                    (0..minute).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    });
                } else {
                    (sleep_start..minute).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    })
                }
            }

            _ => {}
        }
    }

    let mut biggest_sleeper = 0usize;
    let mut sleep_sum = 0usize;

    for (k, v) in result.iter() {
        let sum: usize = v.iter().sum();
        if sum > sleep_sum {
            biggest_sleeper = *k;
            sleep_sum = sum;
        }
    }

    let sleeper = match result.get(&biggest_sleeper) {
        Some(s) => s,
        None => return Err(From::from("could not find two correct box ids")),
    };

    let max_minute = sleeper
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
        .ok_or("Could not find the maximum entry")?;

    Ok(biggest_sleeper * max_minute)
}

fn extract_date(line: &str) -> Result<&str> {
    let date = line
        .split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(date, _)| date.trim())
        .ok_or("Invalid format: date not found in brackets")?;

    Ok(date)
}

fn parse_line(line: &str) -> Result<(usize, &str)> {
    let minute_hour = line
        .split_whitespace()
        .nth(1)
        .ok_or("Can't split on first whitespace")?;
    let minute_str = minute_hour
        .trim_end_matches(']')
        .split(':')
        .nth(1)
        .ok_or("Can't split on colon")?;
    let minute = minute_str
        .parse::<usize>()
        .map_err(|_| "Failed to parse into i32".to_string())?;
    let action = line
        .split("] ")
        .nth(1)
        .ok_or("Can't split on second whitespace")?;

    Ok((minute, action))
}
