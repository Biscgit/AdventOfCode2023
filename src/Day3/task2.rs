use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FoundStar {
    value: u32,
    star_x: usize,
    star_y: usize,
}

impl FoundStar {
    fn new(value: u32, star_x: usize, star_y: usize) -> FoundStar {
        FoundStar {
            value,
            star_x,
            star_y,
        }
    }
}

fn main() {
    let file_path = "src/Day3/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let all_lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut found_stars: Vec<FoundStar> = Vec::new();

    for (index, line) in all_lines.iter().enumerate() {
        // create a vector with the numbers
        let chars: Vec<char> = line.chars().collect();
        let mut numbers: Vec<&str> = line.split(|c: char| !c.is_numeric()).collect();
        numbers.retain(|&s| !s.is_empty());

        // create a vector with the number's indexes
        let mut indexes: Vec<usize> = Vec::new();
        let mut can_save = true;
        for (i, char) in chars.iter().enumerate() {
            if char.is_numeric() {
                if can_save {
                    indexes.push(i);
                }
                can_save = false;
            } else {
                can_save = true;
            }
        }

        // search for numbers with * symbol
        for line_index in indexes {
            // get the next number
            let number = numbers.remove(0);

            // check line above
            if index > 0 {
                if let Some(next_line) = all_lines.get(index - 1) {
                    let next_line_chars: Vec<char> = next_line.chars().collect();
                    let line_index = line_index as i32;

                    for x in (line_index - 1)..=(line_index + (number.len() as i32)) {
                        if let Some(char) = next_line_chars.get(x as usize) {
                            if *char == '*' {
                                found_stars.push(
                                    FoundStar::new(
                                        number.parse::<u32>().unwrap(),
                                        x as usize,
                                        index - 1usize,
                                    )
                                )
                            }
                        }
                    }
                }
            }

            // check line below
            if let Some(next_line) = all_lines.get(index + 1) {
                let next_line_chars: Vec<char> = next_line.chars().collect();
                let line_index = line_index as i32;

                for x in (line_index - 1)..=(line_index + (number.len() as i32)) {
                    if let Some(char) = next_line_chars.get(x as usize) {
                        if *char == '*' {
                            found_stars.push(
                                FoundStar::new(
                                    number.parse::<u32>().unwrap(),
                                    x as usize,
                                    index + 1usize,
                                )
                            )
                        }
                    }
                }
            }

            // check before and after
            let line_chars: Vec<char> = line.chars().collect();

            if line_index > 0 {
                if let Some(char) = line_chars.get(line_index - 1) {
                    if *char == '*' {
                        found_stars.push(
                            FoundStar::new(
                                number.parse::<u32>().unwrap(),
                                line_index - 1,
                                index,
                            )
                        )
                    }
                }
            }

            if let Some(char) = line_chars.get(line_index + number.len()) {
                if *char == '*' {
                    found_stars.push(
                        FoundStar::new(
                            number.parse::<u32>().unwrap(),
                            line_index + number.len(),
                            index,
                        )
                    )
                }
            }

            // check below -1 +1 and right!, for another number check above and left
        }
    }

    // find corresponding numbers
    let mut total_sum = 0;
    let mut used_values: Vec<usize> = Vec::new();

    // compare every entry for common star "reference" (cannot be the same)
    for (i, finding) in (&found_stars).iter().enumerate() {
        'compare: for (j, to_compare) in (&found_stars).iter().enumerate() {
            if finding.star_x == to_compare.star_x
                && finding.star_y == to_compare.star_y
                && i != j {

                // also can only be used once -> indexes stored
                for x in &used_values {
                    if i == *x || j == *x {
                        continue 'compare;
                    }
                }

                used_values.push(i);
                used_values.push(j);

                let product = finding.value * to_compare.value;
                total_sum += product;
            }
        }
    }

    println!("The total sum is {total_sum}")
}
