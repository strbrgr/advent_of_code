use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input/final.txt").expect("Could not read file");
    let result_1 = part_1(&input);
    let result_2 = part_2(&input);
    println!("Result for Part 1 is {}", result_1);
    println!("Result for Part 2 is {}", result_2);
}

fn part_1(input: &str) -> i32 {
    let mut two = 0;
    let mut three = 0;
    for line in input.lines() {
        let mut dic = HashMap::new();
        let mut set = HashSet::new();

        for ch in line.chars() {
            dic.entry(ch).and_modify(|ch| *ch += 1).or_insert(1);
        }

        for count in dic.values() {
            if *count == 2 {
                set.insert(2);
            }

            if *count == 3 {
                set.insert(3);
            }
        }

        for results in set.iter() {
            match *results {
                2 => two += 1,
                3 => three += 1,
                _ => println!("Don't care about the value."),
            }
        }
    }
    two * three
}

fn part_2(input: &str) -> String {
    let mut result = String::from("");
    let mut input_content = Vec::new();
    for line in input.lines() {
        input_content.push(line);
    }
    input_content.sort();

    for n in 0..input_content.len() - 1 {
        let a = input_content[n];
        let b = input_content[n + 1];
        let count = a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count();

        if count == 1 {
            result = a
                .chars()
                .zip(b.chars())
                .filter(|&(a, b)| a == b)
                .map(|(char1, _)| char1)
                .collect();
        }
    }

    result
}
