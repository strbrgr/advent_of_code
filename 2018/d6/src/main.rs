use std::{error::Error, fs, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

// Finite vs infinite?
// Use Manhattan distance to determine each location's closest coordinate
// Does it work with a grid that is bound to the coordinates size?
//
// I have a certain amount of coordinates (input).
// Each point on the grid, keeps track of its own position
// and has a manhattan distance to other coordinates, and keeps track of the smallest
// If there is more than two matching, we place a dot
// Use 2d array
//
//  Setup function:
//  find grid size by getting biggest x and y
//  create 2d array with that
//  TODO:
//  loop over each entry in grid, calculate manhattan distance from each point to each other
//  coordinate, keep track of smallest coordinates char
//  write enum value to grid, enum value is either character or dot
//
// Part 1 should start with the filled out grid and then loop over and do a count
fn main() -> Result<()> {
    let input = fs::read_to_string("input/test.txt")?;
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);
    let mut coordinates: Vec<Vec<usize>> = Vec::new();

    // Get Grid values
    for line in input.lines() {
        let coordinate = line
            .split(", ")
            .map(|s| s.trim().parse())
            .collect::<std::result::Result<Vec<usize>, _>>()?;

        if coordinate.len() == 2 {
            let (x, y) = (coordinate[0], coordinate[1]);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    // Create grid with default values
    let grid: Vec<Vec<String>> = vec![vec![String::new(); max_x]; max_y];
    println!("{:?}", grid);
    Ok(())
}

// fn manhattan_distance
