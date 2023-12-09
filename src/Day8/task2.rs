use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_lcm(numbers: Vec<u64>) -> Option<u64> {
    if numbers.is_empty() {
        return None;
    }

    // Helper function to calculate the GCD
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    // Calculate the LCM for two numbers
    fn lcm(a: u64, b: u64) -> u64{
        a * b / gcd(a, b)
    }

    // Calculate the LCM for the entire vector
    let result = numbers.iter().fold(numbers[0], |acc, &num| lcm(acc, num));
    Some(result)
}

fn main() {
    let file_path = "src/Day8/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let lines = reader.lines();
    let lines: Vec<String> = lines
        .filter_map(|line| line.ok()) // Filter out any lines with errors
        .collect();

    let path: Vec<char> = lines.get(0).unwrap().chars().collect();

    // creating paths (hashmap does not work!! i dont know why, even though it is the same key it cannot find)
    let mut source: Vec<String> = Vec::new();
    let mut destination: Vec<(String, String)> = Vec::new();

    for line in (&lines).iter().skip(2) {
        let parts: Vec<&str> = line.split("=").collect();

        if let [name, rest] = parts.as_slice() {
            let name = name.chars().filter(|&c| c.is_alphanumeric()).collect::<String>();

            let res: Vec<&str> = rest.split(",").collect();
            let left: String = res[0].chars().filter(|&c| c.is_alphanumeric()).collect();
            let right: String = res[1].chars().filter(|&c| c.is_alphanumeric()).collect();

            source.push(name);
            destination.push((left, right));
        }
    }

    // run paths
    let mut starting: Vec<&str> = Vec::new();
    let mut steps: Vec<u64> = Vec::new();

    for n in &source {
        if n.ends_with("A") {
            starting.push(n);
        }
    }

    let length = starting.len();

    for node in 0..length {
        let mut counter: usize = 0;
        let mut current = starting[node];

        loop {
            let mut index = usize::MAX;
            for (i, x) in source.iter().enumerate() {
                if (*x).as_str() == current {
                    index = i;
                }
            }

            match path.get(&counter % path.len()) {
                Some('L') => { current = destination[index].0.as_str() }
                Some('R') => { current = destination[index].1.as_str() }
                _ => panic!()
            }

            counter += 1;

            if current.ends_with("Z") {
                steps.push(counter as u64);
                break;
            }
        }
    }

    // calculate LCM for vales
    let result = calculate_lcm(steps).unwrap();
    println!("{result} steps are needed");
}
