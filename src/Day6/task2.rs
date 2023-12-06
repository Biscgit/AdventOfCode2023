use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "src/Day6/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // initialize values
    let times = lines.next().unwrap().unwrap();
    let times = times
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distances = lines.next().unwrap().unwrap();
    let distances = distances
        .split(":")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    // iterate over times and distances
    let mut wins = 0;

    for x in 1..times {
        let traveled = (times - x) * (x);

        if traveled > distances {
            wins += 1;
        }
    }

    println!("The total product is {wins}");
}
