use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file_path = "src/Day4/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let total_lines = lines.len();

    let mut worth: Vec<usize> = vec![1; total_lines];

    for (index, line) in lines.into_iter().enumerate() {
        // preparing line
        let parts: Vec<&str> = line.split(':').collect();
        let parts: Vec<&str> = parts.get(1).unwrap().split('|').collect();

        // getting numbers
        let mut winning: Vec<&str> = parts.get(0).unwrap().split(|c: char| !c.is_numeric()).collect();
        let mut chosen: Vec<&str> = parts.get(1).unwrap().split(|c: char| !c.is_numeric()).collect();

        // removing empty strings
        winning.retain(|s| !s.is_empty());
        chosen.retain(|s| !s.is_empty());

        // worth of following cards
        let mut matches: usize = 0;

        for x in &winning {
            for y in &chosen {
                if x == y && (index + matches) < total_lines {
                    matches += 1;
                    worth[index + matches] += worth[index];
                }
            }
        }
    }

    let sum: usize = (&worth).iter().sum();
    println!("The total sum is {}", sum);
}
