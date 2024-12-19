use std::{collections::HashMap, error::Error, fs, result, u8};

type Result<T> = result::Result<T, Box<dyn Error>>;

type StepMap = HashMap<char, Vec<char>>;

#[derive(Copy, Clone, Debug)]
struct Worker {
    remaining_seconds: Option<u8>,
    current_char: Option<char>,
}

impl Worker {
    fn new() -> Self {
        Worker {
            remaining_seconds: None,
            current_char: None,
        }
    }
    fn busy(&mut self) -> bool {
        if let Some(remaining) = self.remaining_seconds {
            remaining >= 1
        } else {
            false
        }
    }
    fn work(&mut self) {
        if let Some(mut remaining) = self.remaining_seconds {
            remaining -= 1;
        }
    }
}

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
    match part_2(&incoming, &outgoing) {
        Ok(result) => println!("Part 2 result: {}", result),
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

fn part_2(incoming: &StepMap, outgoing: &StepMap) -> Result<i32> {
    let mut worker = Vec::new();
    let mut available_steps = Vec::new();
    let mut completed_steps = Vec::new();
    let mut result = 0;

    // Find all steps that have no prerequisites (i.e., no incoming edges)
    for k in outgoing.keys() {
        if !incoming.contains_key(k) {
            available_steps.push(*k);
        }
    }

    // Sort available steps alphabetically
    available_steps.sort();

    // Initialize workers
    for _ in 0..5 {
        worker.push(Worker {
            current_char: None,
            remaining_seconds: None,
        });
    }

    // Loop until there are no more tasks to complete
    while !available_steps.is_empty() || worker.iter_mut().any(|w| w.busy()) {
        // Assign tasks to idle workers
        for w in worker.iter_mut() {
            if let Some(remaining_seconds) = w.remaining_seconds {
                if remaining_seconds == 0 {
                    if let Some(step) = w.current_char {
                        completed_steps.push(step);
                    }
                    w.current_char = None;
                    w.remaining_seconds = None;
                }
            }

            if w.current_char.is_none() && !available_steps.is_empty() {
                let next_step = available_steps.remove(0);
                w.current_char = Some(next_step);
                w.remaining_seconds = Some(60 + char_to_idx(next_step as u8));
            }
        }

        // Process workers working on tasks
        for w in worker.iter_mut() {
            if let Some(mut seconds) = w.remaining_seconds {
                seconds -= 1;
                result += 1; // Each second taken
                w.remaining_seconds = Some(seconds);
            }
        }

        // Check completed tasks and add new available steps
        for step in completed_steps.iter() {
            if let Some(next_steps) = outgoing.get(step) {
                for &next_step in next_steps {
                    if let Some(prereqs) = incoming.get(&next_step) {
                        if prereqs.iter().all(|p| completed_steps.contains(p)) {
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

        // Sort available steps alphabetically
        available_steps.sort();
    }

    Ok(result)
}

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

fn at_least_one_worker_busy(worker: &mut [Worker]) -> bool {
    worker.iter_mut().any(|w| w.busy())
}

fn all_worker_busy(worker: &mut [Worker]) -> bool {
    worker.iter_mut().all(|w| w.busy())
}

fn char_to_idx(char: u8) -> u8 {
    char - b'A' + 1
}
