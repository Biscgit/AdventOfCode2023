use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct SeedIterator {
    ranges: Vec<(u64, u64)>,
    length: isize,
}

impl SeedIterator {
    fn new(line: String) -> Self {
        let first_line = line.as_str();
        let range_seeds = first_line
            .split(":")
            .collect::<Vec<&str>>()
            .get(1).unwrap()
            .split(" ")
            .collect::<Vec<&str>>();

        let range_seeds: Vec<u64> = range_seeds
            .iter()
            .filter_map(|&s| s.parse().ok())
            .collect();

        let ranges: Vec<(u64, u64)> = range_seeds.chunks(2)
            .map(|chunk| match chunk {
                &[a, b] => (a, b),
                _ => unreachable!(),
            })
            .collect();

        let length: u64 = (&ranges).iter().map(|&(_, second)| second).sum();
        let length = length as isize;

        SeedIterator {
            ranges,
            length,
        }
    }
}

impl Iterator for SeedIterator {
    type Item = (isize, u64);

    fn next(&mut self) -> Option<Self::Item> {
        return if let Some(pair) = self.ranges.pop() {
            if pair.1 > 0 {
                self.ranges.push((pair.0, pair.1 - 1));
            }

            self.length -= 1;
            Some((self.length, pair.0 + pair.1))
        } else {
            None
        };
    }
}

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
    println!("Loading sources...");
    let file_path = "src/Day5/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);

    let mut all_paths: Vec<Vec<Path>> = Vec::new();
    let mut lines = reader.lines();

    // get the seeds
    println!("Creating seed iterator...");
    let first_line = lines.next().unwrap().unwrap();
    let seed_iterator = SeedIterator::new(first_line);

    // generate paths
    println!("Generating paths...");
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
    println!("Calculating {} possibilities...", seed_iterator.length);
    let mut lowest: u64 = u64::MAX;

    let total_length = seed_iterator.length;
    let mut next = 0;

    for (l, seed) in seed_iterator {
        // simple progress display
        if next < ((total_length - l) * 100 / total_length) {
            println!("{:>4}% done", next);
            next += 5;
        }

        // check ever path layer once
        let mut seed = seed;

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

        lowest = min(lowest, seed);
    }

    println!("{:>4}% done", 100);
    println!("The lowest destination is {lowest}");
}
