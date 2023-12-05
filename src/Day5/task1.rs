use std::fs::File;
use std::io::{BufRead, BufReader};

struct Path {
    destination: u64,
    source: u64,
    range: u64,
}

impl Path {
    fn new(values: &(Option<&str>, Option<&str>, Option<&str>)) -> Path {
        // values to u64
        Path {
            destination: values.0.unwrap().parse::<u64>().unwrap(),
            source: values.1.unwrap().parse::<u64>().unwrap(),
            range: values.2.unwrap().parse::<u64>().unwrap(),
        }
    }

    fn check_path(&self, value: u64) -> Option<u64> {
        if (self.source <= value) && (value < (self.source + self.range)) {
            return Some(value + self.destination - self.source);
        }
        return None;
    }
}


fn main() {
    let file_path = "src/Day5/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    let mut all_paths: Vec<Vec<Path>> = Vec::new();
    let mut lines = reader.lines();

    // get the seeds
    let first_line = lines.next().unwrap().unwrap();
    let first_line = first_line.as_str();
    let seeds = first_line
        .split(":")
        .collect::<Vec<&str>>()
        .get(1).unwrap()
        .split(" ")
        .collect::<Vec<&str>>();

    let seeds: Vec<u64> = seeds
        .iter()
        .filter_map(|&s| s.parse().ok())
        .collect();

    // generate paths
    for line in lines {
        let line = line.unwrap();

        // load in the paths
        if line.is_empty() { continue; }
        if line.contains("map") {
            all_paths.push(Vec::new());
            continue;
        }

        // push into paths
        let mut values = line.split(" ");
        if let Some(mut list) = all_paths.pop() {
            list.push(
                Path::new(&(values.next(), values.next(), values.next()))
            );
            all_paths.push(list);
        }
    }

    // calculate paths
    let mut results: Vec<u64> = Vec::new();
    for seed in &seeds {
        // check ever path layer once
        let mut seed = *seed;

        'layer_loop: for layer in all_paths.iter() {
            // check every option
            for option in layer {
                // dont update if no match found
                if let Some(new_val) = option.check_path(seed) {
                    seed = new_val;
                    continue 'layer_loop;
                }
            }
        }

        results.push(seed);
    }

    let lowest = results.iter().min().unwrap();
    println!("The lowest destination is {lowest}");
}
