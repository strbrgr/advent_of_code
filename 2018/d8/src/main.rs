use std::{error::Error, fs, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?;

    let numbers: Vec<usize> = input
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();

    let mut starting_index = 0usize;
    let root = parse_tree(&numbers, &mut starting_index);
    println!("{:#?}", root);

    let part_1 = part_1(&root);
    println!("{part_1}");

    let part_2 = part_2(&root);
    println!("{part_2}");
    Ok(())
}

// Returned node is the starting node
fn parse_tree(input: &Vec<usize>, starting_index: &mut usize) -> Node {
    let number_children = input.get(*starting_index);
    // Advance index after
    *starting_index += 1;
    let number_metadata = input.get(*starting_index);
    *starting_index += 1;

    let mut root = Node {
        children: vec![],
        metadata: vec![],
    };

    // Recursively parses child nodes and adds them to children vector
    // Only runs when number_children > 0
    if let Some(num) = number_children {
        for _ in 0..*num {
            root.children.push(parse_tree(input, starting_index));
        }
    }

    if let Some(num) = number_metadata {
        for _ in 0..*num {
            if let Some(idx) = input.get(*starting_index) {
                root.metadata.push(*idx);
                *starting_index += 1;
            }
        }
    }

    root
}

fn part_1(root: &Node) -> usize {
    let metadata_sum: usize = root.metadata.iter().sum();

    // Will be 0 when there are no children
    // running .map on an empty iterator is fine
    let children_sum: usize = root.children.iter().map(part_1).sum();

    metadata_sum + children_sum
}

fn part_2(root: &Node) -> usize {
    if root.children.is_empty() {
        root.metadata.iter().sum::<usize>()
    } else {
        root.metadata
            .iter()
            .filter_map(|i| root.children.get(i - 1))
            .map(part_2)
            .sum()
    }
}
