use std::{collections::HashMap, error::Error, fs, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

type StepMap = HashMap<char, Vec<char>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?;
    let mut incoming: StepMap = HashMap::new();
    let mut outgoing: StepMap = HashMap::new();

    for l in input.lines() {
        let (points_to, pointed_at) = parse_line(l);
        // The chars that have incoming "pointers"
        incoming
            .entry(pointed_at)
            .and_modify(|e| {
                e.push(points_to);
            })
            .or_insert_with(|| vec![points_to]);
        // The chars that have outgoing "pointers"
        outgoing
            .entry(points_to)
            .and_modify(|e| {
                e.push(pointed_at);
            })
            .or_insert_with(|| vec![pointed_at]);
    }

    match part_1(&incoming, &outgoing) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => return Err(err),
    }
    Ok(())
}

fn part_1(incoming: &StepMap, outgoing: &StepMap) -> Result<String> {
    let mut available_steps = Vec::new();
    let mut completed_steps = Vec::new();

    // Find all steps that have no prerequisites
    for k in outgoing.keys() {
        if !incoming.contains_key(k) {
            available_steps.push(*k);
        }
    }

    while !available_steps.is_empty() {
        // Sort available steps to always choose alphabetically first
        available_steps.sort();

        // Take the first available step
        let current_step = available_steps.remove(0);
        completed_steps.push(current_step);

        // If this step has outgoing steps, check which can now be completed
        if let Some(next_steps) = outgoing.get(&current_step) {
            for &next_step in next_steps {
                // Check if all prerequisites for this next step are completed
                if let Some(prereqs) = incoming.get(&next_step) {
                    if prereqs.iter().all(|p| completed_steps.contains(p)) {
                        // Only add if not already in available or completed steps
                        if !available_steps.contains(&next_step)
                            && !completed_steps.contains(&next_step)
                        {
                            available_steps.push(next_step);
                        }
                    }
                }
            }
        }
    }

    Ok(completed_steps.iter().collect::<String>())
}

// fn part_2() {}

// Returns character tuple, at 1: &str from which we point, at 2: &str pointed at
// Index' are guaranteed to exist
fn parse_line(line: &str) -> (char, char) {
    let a = line[5..6]
        .chars()
        .next()
        .expect("Expected at least 6 characters in the line");
    let b = line[36..37]
        .chars()
        .next()
        .expect("Expected at least 36 characters in the line");

    (a, b)
}
