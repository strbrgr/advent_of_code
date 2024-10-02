use std::{collections::HashSet, fs::read_to_string};

// TODO: Create stryct for a Cell in the matrix
// TODO: Impl the new() and add_id() for struct
// TODO: Split_once()
// TODO: Proper error handling with split once and parse
// TODO: Change direct access via index values, use tuples and destructuring

fn main() {
    // TODO: Spread the logic into two parts and create matrix in here
    let input = read_to_string("input/final.txt").expect("Could not read file");
    let part_1 = part_1(&input);
    println!("Result for part 1: {}", part_1);
}

fn part_1(input: &str) -> i32 {
    let mut matrix: Vec<Vec<(i32, HashSet<&str>)>> = vec![vec![(0, HashSet::new()); 1000]; 1000];
    let mut count = 0;

    for l in input.lines() {
        let subset: Vec<&str> = l.split(" @ ").collect();
        let id = &subset[0][1..];
        let coordinates: Vec<&str> = subset[1].split(": ").collect();
        let distances: Vec<&str> = coordinates[0].split(',').collect();
        let sizes: Vec<&str> = coordinates[1].split('x').collect();

        let column_start = distances[0].parse::<usize>().expect("Could not convert");
        let row_start = distances[1].parse::<usize>().expect("Could not convert");
        let width = sizes[0].parse::<usize>().expect("Could not convert");
        let height = sizes[1].parse::<usize>().expect("Could not convert");
        let column_end = column_start + width;
        let row_end = row_start + height;

        (column_start..column_end).for_each(|i| {
            (row_start..row_end).for_each(|j| {
                matrix[i][j].0 += 1;
                matrix[i][j].1.insert(id);
            })
        });
    }

    let mut ones: HashSet<&str> = HashSet::new();
    let mut twos: HashSet<&str> = HashSet::new();

    for row in &matrix {
        for element in row {
            let count_matrix = element.0;
            if count_matrix >= 2 {
                count += 1;
                twos.extend(&element.1);
            }
            if count_matrix < 2 {
                ones.extend(&element.1);
            }
        }
    }

    let result: HashSet<_> = ones.difference(&twos).collect();

    println!("{:?}", result);
    count
}
