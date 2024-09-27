use std::collections::HashSet;
use std::fs;

fn main() {
    let mut set = HashSet::new();
    let message: String =
        fs::read_to_string("input/final.txt").expect("Could not read content of file.");
    let mut sum = 0;
    set.insert(0);

    loop {
        for l in message.lines() {
            sum += l.parse::<i32>().expect("Could not parse to type i32");

            if set.contains(&sum) {
                println!("{}", sum);
                return;
            }

            set.insert(sum);
        }
    }
}
