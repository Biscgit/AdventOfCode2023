use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file_path = "src/task1.txt";
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut total_sum = 0;

    for line in reader.lines() {
        let mut chars: Vec<char> = line?.chars().collect();
        let mut line_value = 0;

        for x in &chars {
            if x.is_numeric() {
                line_value += 10 * x.to_digit(10).unwrap();
                break;
            }
        }

        chars.reverse();

        for x in &chars {
            if x.is_numeric() {
                line_value += x.to_digit(10).unwrap();
                break;
            }
        }

        total_sum += line_value;
    }

    println!("The total sum is {}", total_sum);

    Ok(())
}
