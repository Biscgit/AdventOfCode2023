use std::fs::File;
use std::io::{BufRead, BufReader};

fn color_to_value(color: &str) -> Option<i32> {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    match color.to_lowercase().as_str() {
        "red" => Some(12),
        "green" => Some(13),
        "blue" => Some(14),
        _ => None
    }
}

fn check_valid(slice: &str) -> bool {
    for part in slice.split(',') {
        let part: Vec<&str> = part.trim().split(' ').collect();

        if let [count, color] = part.as_slice() {
            let count = count.parse::<i32>().unwrap();

            // invalid if more cubes than allowed
            if count > color_to_value(color).unwrap() {
                return false;
            }
        } else {
            panic!("Invalid input format!");
        }
    }

    return true;
}

fn main() {
    let file_path = "src/Day2/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let mut total_ids = 0;

    // looping over every line
    'line: for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line: Vec<&str> = line.as_str().split(&[':', ';'][..]).collect();

        // checking each combination, dropping the "Game X:" part
        for slice in line[1..].to_vec() {
            if !check_valid(slice) {
                continue 'line;
            }
        }

        // adding line identifier
        total_ids += index + 1;
    }

    println!("Total sum of correct lines {total_ids}");
}
