use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "src/Day6/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // initialize values
    let times = lines.next().unwrap().unwrap();
    let times = times
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|&s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = lines.next().unwrap().unwrap();
    let distances = distances
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|&s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    // iterate over times and distances
    let mut result: u32 = 1;

    for (time, record) in times.iter().zip(distances.iter()) {
        let mut wins = 0;

        for x in 1..*time {
            let traveled = (time - x) * (x);

            if traveled > *record {
                wins += 1;
            }
        }

        result *= wins;
    }

    println!("The total product is {result}");
}
