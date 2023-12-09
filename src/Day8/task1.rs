use std::fs::File;
use std::io::{BufRead, BufReader};

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
            let name = name.chars().filter(|&c| c.is_alphabetic()).collect::<String>();

            let res: Vec<&str> = rest.split(",").collect();
            let left: String = res[0].chars().filter(|&c| c.is_alphabetic()).collect();
            let right: String = res[1].chars().filter(|&c| c.is_alphabetic()).collect();

            source.push(name);
            destination.push((left, right));
        }
    }

    // run paths
    let mut counter: usize = 0;
    let mut current = "AAA";

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
        if current == "ZZZ" { break; }
    }

    println!("The total needed steps are {counter}");
}