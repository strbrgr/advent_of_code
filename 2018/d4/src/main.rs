use std::error::Error;
use std::result;
use std::{collections::HashMap, fs};

type Result<T> = result::Result<T, Box<dyn Error>>;
type GuardSleepFrequency = HashMap<usize, [usize; 60]>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?;
    let mut lines: Vec<&str> = input.lines().collect();
    // TODO: Handle expect
    lines.sort_by_key(|line| extract_date(line).expect("Invalid format"));

    // Sleep stats keeps track of Guard Id and minutes sleeping and their count
    let mut sleep_stats: GuardSleepFrequency = HashMap::new();
    let mut sleep_start = 0usize;
    let mut id = 0usize;

    // Loops over each line and keeps track of a guard sleep in sleep_stats
    for line in &lines {
        let (current_minute, action) = parse_line(line)?;
        match action {
            _ if action.starts_with("Guard ") => {
                let (_, id_content) = action
                    .split_once(" #")
                    .ok_or("Could not split Guard info")?;
                let (guard_id, _) = id_content.split_once(' ').ok_or("Could not split id")?;
                id = guard_id.parse::<usize>()?;
            }

            _ if action.starts_with("falls asleep") => {
                sleep_start = current_minute;
            }

            _ if action.starts_with("wakes up") => {
                let minute_sleep_count = sleep_stats.entry(id).or_insert([0; 60]);

                if current_minute < sleep_start {
                    (sleep_start..60).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    });

                    (0..current_minute).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    });
                } else {
                    (sleep_start..current_minute).for_each(|m| {
                        minute_sleep_count[m] += 1;
                    })
                }
            }

            _ => {}
        }
    }

    match part_1(&sleep_stats) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => println!("There was an error in Part 1: {}", err),
    };

    Ok(())
}

fn part_1(frequency: &GuardSleepFrequency) -> Result<usize> {
    let mut max_guard_sleeping = 0usize;
    let mut sleep_sum = 0usize;

    for (k, v) in frequency.iter() {
        let current_sum: usize = v.iter().sum();
        if current_sum > sleep_sum {
            max_guard_sleeping = *k;
            sleep_sum = current_sum;
        }
    }

    let minute_frequency = match frequency.get(&max_guard_sleeping) {
        Some(freq) => freq,
        None => return Err(From::from("Could not retrieve frequency for a guard")),
    };

    let max_minute = minute_frequency
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
        .ok_or("Could not get maximum minute asleep")?;

    Ok(max_guard_sleeping * max_minute)
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
