use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_next(sequence: Vec<i32>) -> i32 {
    // start recursion
    sequence[0] - recursive_next(sequence)
}

fn recursive_next(sequence: Vec<i32>) -> i32 {
    let mut differences: Vec<i32> = Vec::new();

    // get all difference
    for pair in sequence.windows(2) {
        differences.push(pair[1] - pair[0])
    }

    // check if every value is 0
    return if differences.iter().all(|&x| x == 0) {
        0
    } else {
        differences[0] - recursive_next(differences)
    };
}

fn main() {
    let file_path = "src/Day9/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let values = line
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|&s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let next = get_next(values);
        sum += next;
    }

    println!("The sum of all values is {sum}");
}
