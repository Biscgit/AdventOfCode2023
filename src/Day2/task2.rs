use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_minimum<'a>(map: &mut HashMap<&'a str, u32>, slice: &'a str) {
    for part in slice.split(',') {
        let part: Vec<&str> = part.trim().split(' ').collect();

        if let [count, color] = part.as_slice() {
            let count = count.parse::<u32>().unwrap();

            // insert or update with largest value
            match map.get(color) {
                Some(value) => {
                    if count > *value { map.insert(color, count); }
                }
                None => { map.insert(color, count); }
            }
        } else {
            panic!("Invalid input format!");
        }
    }
}

fn main() {
    let file_path = "src/Day2/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut total_sum = 0;

    // looping over every line
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.as_str().split(&[':', ';'][..]).collect();

        // checking each combination, dropping the "Game X:" part
        let mut min_map: HashMap<&str, u32> = HashMap::new();

        for slice in line[1..].to_vec() {
            calculate_minimum(&mut min_map, slice);
        }

        let sum = min_map.get("red").unwrap_or(&0)
            * min_map.get("green").unwrap_or(&0)
            * min_map.get("blue").unwrap_or(&0);

        total_sum += sum;
    }

    println!("Total sum of correct lines {total_sum}");
}
