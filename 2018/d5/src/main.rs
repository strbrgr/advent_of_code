use std::{error, fs};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?.trim().to_string();

    match part_1(&input) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => return Err(err),
    }

    match part_2(&input) {
        Ok(result) => println!("Part 2 result: {}", result),
        Err(err) => return Err(err),
    }
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let cleaned_stack_length = react(input);
    Ok(cleaned_stack_length)
}

fn part_2(input: &str) -> Result<i32> {
    let mut result = i32::MAX;

    for i in 0..26 {
        let mut stack = String::from("");
        let lower = (b'a' + i) as char;
        let upper = (b'A' + i) as char;

        for char in input.chars() {
            if char != lower && char != upper {
                stack.push(char);
            }
        }

        let length = react(&stack);

        let length = i32::try_from(length).unwrap();
        if length < result {
            result = length;
        }
    }

    Ok(result)
}

fn react(input: &str) -> usize {
    let mut stack = String::from("");

    for char in input.chars() {
        if let Some(top) = stack.chars().last() {
            if will_units_react(char, top) {
                stack.pop();
            } else {
                stack.push(char);
            }
        } else {
            stack.push(char);
        }
    }
    stack.len()
}

fn will_units_react(x: char, y: char) -> bool {
    (x.is_lowercase() && y.is_uppercase() && x.to_ascii_uppercase() == y)
        || (x.is_uppercase() && y.is_lowercase() && x == y.to_ascii_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn will_units_react_test() {
        assert!(will_units_react('c', 'C'));
        assert!(will_units_react('C', 'c'));
        assert!(will_units_react('B', 'b'));
        assert!(will_units_react('b', 'B'));
    }

    #[test]
    fn will_units_not_react_test() {
        assert!(!will_units_react('a', 'a'));
        assert!(!will_units_react('B', 'B'));
        assert!(!will_units_react('a', 'B'));
        assert!(!will_units_react('A', 'b'));
    }
}
