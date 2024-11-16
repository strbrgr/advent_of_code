use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    error::Error,
    fs, result,
};

type Result<T> = result::Result<T, Box<dyn Error>>;
type Grid<T> = Vec<Vec<T>>;

// Part 1 should start with the filled out grid and then loop over and do a count
fn main() -> Result<()> {
    let input = fs::read_to_string("input/final.txt")?;
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);
    // Holds vectors of coordinates
    let mut coordinates: Vec<Vec<i32>> = Vec::new();

    // Creat Grid
    // TODO: Hanlde parsing error better
    for line in input.lines() {
        let coordinate = line
            .split(", ")
            .map(|s| s.trim().parse())
            .collect::<std::result::Result<Vec<i32>, _>>()?;

        // Find the maximum x/y value so that we can create the grid
        if coordinate.len() == 2 {
            // I'm okay with unwrap after checking for length
            let (x, y) =
                (coordinate.first().unwrap(), coordinate.get(1).unwrap());
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        coordinates.push(coordinate);
    }

    // Adds a buffer
    max_x += 1;
    max_y += 1;
    let mut grid: Grid<String> =
        vec![vec![String::new(); max_x as usize]; max_y as usize];

    match part_1(&mut grid, &coordinates) {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(err) => return Err(err),
    }
    Ok(())
}

fn part_1(grid: &mut Grid<String>, coordinates: &Grid<i32>) -> Result<i32> {
    let mut infinite_values = HashSet::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut minimum_distance = i32::MAX;
            let mut minimum_index = None;
            let mut conflict = false;

            for (i, el) in coordinates.iter().enumerate() {
                let manhattan_distance =
                    (row as i32 - el[1]).abs() + (col as i32 - el[0]).abs();

                match manhattan_distance.cmp(&minimum_distance) {
                    Ordering::Less => {
                        minimum_distance = manhattan_distance;
                        minimum_index = Some(i);
                        conflict = false;
                    }
                    Ordering::Equal => {
                        conflict = true;
                    }
                    Ordering::Greater => {
                        // Do nothing
                    }
                }
            }

            if conflict {
                grid[row][col] = String::from(".");
            } else if let Some(index) = minimum_index {
                let coordinate = (b'A' + index as u8) as char;
                grid[row][col] = coordinate.to_string();
            }

            // Check which of the values are infinte
            if row == 0
                || row == grid.len() - 1
                || col == 0
                || col == grid[0].len() - 1
            {
                infinite_values.insert(grid[row][col].to_string());
            }
        }
    }

    let mut finite_values: HashMap<String, i32> = HashMap::new();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !infinite_values.contains(&grid[row][col]) {
                *finite_values
                    .entry(grid[row][col].to_string())
                    .or_insert(0) += 1;
            }
        }
    }

    let largest_value = finite_values
        .iter()
        .max_by_key(|&(_, v)| v)
        .ok_or("Could not find an entry")?;

    Ok(*largest_value.1)
}
