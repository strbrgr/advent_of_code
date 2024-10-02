// Caveats: Guard shift can begin at 58. Falls asleep is always followed by wakes up
//          Entries are not in chronological order -> Sort
// Create a struct for a Guard, each guard will be added to a vector
// The HashMap holds key = minute asleep and value = count asleep
//
// Step 1: Sort the input by date and then by hour / minute
// Step 2: Run through the sorted input by line
//   If the Line starts with Guard we run a while (not start with Guard) statement
//      if line starts with falls asleep we track starting minute
//      if line starts with wakes up we track ending minute
//      and run the loop to add entries in the hashmap
// Step 3: Which array has biggest sum of minutes?
// Step 4: Which array has biggst count of minutes?
//
//
//
//   Make enums for asleep, wakes up
//
use std::{error::Error, fs, result};

struct Guard {
    id: i32,
    minute_count: [i32; 60],
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input/final.txt")?;

    let mut lines: Vec<&str> = input.lines().collect();

    lines.sort_by_key(|line| extract_date(line).expect("Invalid format"));

    for line in lines {
        println!("{line}")
    }
    Ok(())
}

fn extract_date(line: &str) -> Result<&str, &'static str> {
    line.split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(date, _)| date.trim())
        .ok_or("Invalid format: date not found in brackets")
}
