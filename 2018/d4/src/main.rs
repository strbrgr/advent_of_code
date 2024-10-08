use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/final.txt")?;

    let mut lines: Vec<&str> = input.lines().collect();

    lines.sort_by_key(|line| extract_date(line).expect("Invalid format"));

    match part_1(&lines) {
        Ok(result) => println!("{result}"),
        Err(err) => println!("{err}"),
    };

    Ok(())
}

fn part_1(lines: &Vec<&str>) -> Result<i32, String> {
    let mut result: HashMap<i32, [i32; 60]> = HashMap::new();
    let mut sleep_start = 0usize;
    let mut id = 0;

    for line in lines {
        let (minute, action) = parse_line(&line)?;

        if action.starts_with("Guard ") {
            // TODO: Improve extracting the id with regex.
            let (_, id_content) = action
                .split_once(" #")
                .ok_or("Could not split Guard info")?;
            let (guard_id, _) = id_content.split_once(' ').ok_or("Could not split id")?;
            // TODO: Avoid expect
            id = guard_id.parse::<i32>().expect("Could not parse");
        } else if action.starts_with("falls asleep") {
            sleep_start = minute;
        } else if action.starts_with("wakes up") {
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
    }

    let mut biggest_sleeper = 0;
    let mut sleep_sum = 0;

    for (k, v) in result.iter() {
        let sum: i32 = v.iter().sum();
        if sum > sleep_sum {
            biggest_sleeper = *k;
            sleep_sum = sum;
        }
    }

    let sleeper = match result.get(&biggest_sleeper) {
        Some(s) => s,
        None => return Err(String::from("Could not get value out of HashMap")),
    };

    let max_minute = match sleeper
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
    {
        Some(max) => max,
        None => return Err(String::from("Could get max_minute")),
    };

    Ok(biggest_sleeper * max_minute as i32)
}

fn extract_date(line: &str) -> Result<&str, &'static str> {
    line.split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(date, _)| date.trim())
        .ok_or("Invalid format: date not found in brackets")
}

// TODO: Better Error Return
fn parse_line(line: &str) -> Result<(usize, &str), String> {
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
