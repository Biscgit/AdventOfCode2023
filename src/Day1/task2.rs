use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn number_to_digit(input: &str) -> Option<i32> {
    match input.to_lowercase().as_str() {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None
    }
}

fn main() -> io::Result<()> {
    let file_path = "src/Day1/input.txt";
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut total_sum = 0;

    for line in reader.lines() {
        // preparing
        let line = line.unwrap();

        let mut chars: Vec<char> = line.chars().collect();
        let mut line_value = 0;

        // finding index of first text number;
        let numbers = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut smallest_index = line.len();
        let mut to_add = 0;

        for number in &numbers[..] {
            if let Some(index) = line.find(number) {
                if smallest_index > index {
                    smallest_index = index;
                    to_add = number_to_digit(number).unwrap();
                }
            }
        }

        // find smallest integer and compare position, chose first
        for (i, x) in chars.iter().enumerate() {
            if x.is_numeric() {
                if i < smallest_index {
                    to_add = x.to_digit(10).unwrap() as i32;
                }
                break;
            }
        }

        // adding number as bigger digit
        line_value += 10 * to_add;

        // repeat from behind with smaller digit

        let mut largest_index = 0;
        let mut to_add = 0;

        for number in &numbers[..] {
            if let Some(index) = line.rfind(number) {
                if largest_index < index {
                    largest_index = index;
                    to_add = number_to_digit(number).unwrap();
                }
            }
        }

        chars.reverse();
        for (i, x) in chars.iter().enumerate() {
            if x.is_numeric() {
                if line.len() - i > largest_index {
                    to_add = x.to_digit(10).unwrap() as i32;
                }
                break;
            }
        }

        line_value += to_add;
        println!("{line_value}");

        // finally sum up line value
        total_sum += line_value;
    }

    println!("The total sum is {}", total_sum);

    Ok(())
}
