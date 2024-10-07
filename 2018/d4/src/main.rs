// Caveats: Guard shift can begin at 58. Falls asleep is always followed by wakes up
//          Entries are not in chronological order -> Sort
// HashMap: key id of guard, value vector or array, size 60, count of minute asleep
//
// Step 1: Sort the input by date and then by hour / minute
// Step 2: Run through the sorted input by line
// Step 3: Parse the log, get timestamp, guard_id, action
//   If the Line starts with Guard we run a while (not start with Guard) statement
//      if line starts with falls asleep we track starting minute
//      if line starts with wakes up we track ending minute
//      and run the loop to add entries in the hashmap
//
// Step 3: Which array has biggest sum of minutes?
// Step 4: Which array has biggst count of minutes?

use std::{env::current_dir, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/final.txt")?;

    let mut lines: Vec<&str> = input.lines().collect();

    lines.sort_by_key(|line| extract_date(line).expect("Invalid format"));

    part_1(&lines);
    Ok(())
}

fn part_1(lines: &Vec<&str>) -> Result<i32, &'static str> {
    for line in lines {
        let mut minutes_asleep = 0;
        let mut sleep_start = 0;
        let mut id = String::new();

        // TODO: Improve parsing, split the logic in separate function
        let (date, info) = line.split_once("] ").ok_or("Error splitting line.")?;
        let (_, minute) = date.split_once(':').ok_or("Error splitting date line")?;
        let current_minute = minute
            .parse::<i32>()
            .expect("Could not parse string to i32 type");

        // TODO: Match statements
        if info.starts_with("Guard ") {
            let (_, id_content) = info.split_once(" #").ok_or("Could not split Guard info")?;
            let (guard_id, _) = id_content.split_once(' ').ok_or("Could not split id")?;
            id = String::from(guard_id);
        }

        if info == "falls asleep" {
            sleep_start = current_minute;
        }

        if info == "wakes up" {
            if sleep_start > current_minute {
                let mut minutes_before_full_hour = 0;
                for _ in sleep_start..=59 {
                    // Needs to update array values
                    minutes_before_full_hour += 1;
                }

                for _ in 0..current_minute {
                    // Needs to update array values
                    minutes_before_full_hour += 1;
                }
            } else {
                // Needs to update array values
                minutes_asleep += current_minute - sleep_start;
            }
        }
    }
    Ok(231)
}

fn extract_date(line: &str) -> Result<&str, &'static str> {
    line.split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(date, _)| date.trim())
        .ok_or("Invalid format: date not found in brackets")
}
