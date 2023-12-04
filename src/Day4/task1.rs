use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file_path = "src/Day4/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut total_sum: u32 = 0;

    for line in reader.lines() {
        // preparing line
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        let parts: Vec<&str> = parts.get(1).unwrap().split('|').collect();

        // getting numbers
        let mut winning: Vec<&str> = parts.get(0).unwrap().split(|c: char| !c.is_numeric()).collect();
        let mut chosen: Vec<&str> = parts.get(1).unwrap().split(|c: char| !c.is_numeric()).collect();

        // removing empty strings
        winning.retain(|s| !s.is_empty());
        chosen.retain(|s| !s.is_empty());

        // calculating for each number
        let mut points: f64 = 0.5;

        for x in &winning {
            for y in &chosen {
                if x == y { points *= 2.0; }
            }
        }

        // summing up points
        total_sum += (points.floor() as u32);
    }

    println!("The total sum is {}", total_sum);
}
