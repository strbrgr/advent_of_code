use std::{error, fs};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?.trim().to_string();

    match part_1(&input) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => return Err(err),
    }
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
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
    Ok(stack.len())
}

// Change return type to return Error type not string
fn get_char(input: &str, position: usize) -> std::result::Result<char, &str> {
    input
        .chars()
        .nth(position)
        .ok_or("Can't get character at passed index.")
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
