use std::{error, fs};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// Manipulate the string directly
// sliding window, whenever there is a match we remove and move both pointers to the left one spot, checj if there is a match we remove.
// If I can't delete them directly, I can overwrite them with a #

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?.trim().to_string();

    part_1(&input)?;

    Ok(())
}

fn part_1(input: &str) -> Result<()> {
    let mut stack = String::new();

    for i in 0..input.len() {
        let mut left = i;
        let mut right = i + 1;
        let left_char = input
            .chars()
            .nth(left)
            .ok_or("Could not find character at index")?;
        let right_char = input
            .chars()
            .nth(right)
            .ok_or("Could not find character at index")?;

        if will_units_react(left_char, right_char) {
            //
        } else {
            stack.push(left_char);
            stack.push(right_char);
        }
    }
    Ok(())
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
        assert!(!will_units_react('a', 'C'));
        assert!(!will_units_react('C', 'a'));
        assert!(!will_units_react('B', 'd'));
        assert!(!will_units_react('d', 'B'));
    }
}
