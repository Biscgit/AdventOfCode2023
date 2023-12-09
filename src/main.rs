use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "src/DayX/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
    }
}