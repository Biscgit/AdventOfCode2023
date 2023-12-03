use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let file_path = "src/Day3/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut total_sum = 0;

    for (index, line) in all_lines.iter().enumerate() {
        // keeping track of line_index in case number occurs multiple times in a line
        let mut line_index: i32 = 0;
        let parts: Vec<&str> = line.split(|c: char| !c.is_numeric()).collect();

        'part_loop: for part in parts {
            // part is a number => check surrounding symbols
            match part.parse::<i32>() {
                Ok(number) => {
                    // line above
                    // get line if possible, check for index first
                    if index > 0 {
                        if let Some(slice) = all_lines.get(index - 1usize) {
                            let slice: Vec<char> = slice.chars().collect();

                            // iterate over every reachable char
                            for x in (line_index - 1)..=(line_index + (part.len() as i32)) {

                                // check if other character than dot
                                if let Some(char) = slice.get(x as usize) {
                                    if *char != '.' {
                                        // add number because valid
                                        total_sum += number;
                                        line_index += (part.len() as i32) + 1;

                                        continue 'part_loop;
                                    }
                                }
                            }
                        }
                    }

                    //before and after
                    let slice: Vec<char> = line.chars().collect();

                    if line_index > 0 {
                        if let Some(char) = slice.get((line_index - 1) as usize) {
                            if *char != '.' {
                                total_sum += number;
                                line_index += (part.len() as i32) + 1;

                                continue 'part_loop;
                            }
                        }
                    }

                    if (line_index as usize) + part.len() < line.len() {
                        if let Some(char) = slice.get((line_index as usize) + part.len()) {
                            if *char != '.' {
                                total_sum += number;
                                line_index += (part.len() as i32) + 1;

                                continue 'part_loop;
                            }
                        }
                    }

                    // line below
                    // get line if possible
                    if index < all_lines.len() {
                        if let Some(slice) = all_lines.get(index + 1usize) {
                            let slice: Vec<char> = slice.chars().collect();

                            // iterate over every reachable char
                            for x in (line_index - 1)..=(line_index + (part.len() as i32)) {
                                // if dot then break
                                if let Some(char) = slice.get(x as usize) {
                                    if *char != '.' {
                                        // add number because valid
                                        total_sum += number;
                                        line_index += (part.len() as i32) + 1;

                                        continue 'part_loop;
                                    }
                                }
                            }
                        }
                    }

                    line_index += (part.len() as i32) + 1;
                }

                Err(_) => { line_index += 1; }
            }
        }
    }

    println!("The total sum is {total_sum}")
}
