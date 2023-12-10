use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "src/Day10/input.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().into_iter().map(|x| x.unwrap()).collect();

    // find starting S
    let mut start = (0, 0);

    for (height, line) in (&lines).iter().enumerate() {
        if let Some(width) = line.find("S") {
            start = (height, width)
        }
    }

    // run through pipes
    let lines: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect::<Vec<char>>()).collect();

    // find the first pipe where to go
    let search_first_pipe = |position: (usize, usize)| -> (usize, usize) {
        // check above, left, right, down
        if ['7', 'F', '|'].contains(&lines[position.0 - 1][position.1]) {
            (position.0 - 1, position.1)
        } else if ['L', 'F', '-'].contains(&lines[position.0][position.1 - 1]) {
            (position.0, position.1 - 1)
        } else if ['J', '7', '-'].contains(&lines[position.0][position.1 + 1]) {
            (position.0, position.1 + 1)
        } else if ['L', 'J', '|'].contains(&lines[position.0 + 1][position.1]) {
            (position.0 + 1, position.1)
        } else {
            unimplemented!()
        }
    };

    // function returns next place to go
    fn get_next(symbol: char, position: (usize, usize), char_position: (usize, usize)) -> (usize, usize) {
        match symbol {
            '|' => {
                // symbol below else above
                if position.0 < char_position.0 {
                    (char_position.0 + 1, char_position.1)
                } else {
                    (char_position.0 - 1, char_position.1)
                }
            }
            '-' => {
                // symbol to the right else left
                if position.1 < char_position.1 {
                    (char_position.0, char_position.1 + 1)
                } else {
                    (char_position.0, char_position.1 - 1)
                }
            }
            'L' => {
                // symbol below else to the left
                if position.0 < char_position.0 {
                    (char_position.0, char_position.1 + 1)
                } else {
                    (char_position.0 - 1, char_position.1)
                }
            }
            'J' => {
                // symbol below else to the right
                if position.0 < char_position.0 {
                    (char_position.0, char_position.1 - 1)
                } else {
                    (char_position.0 - 1, char_position.1)
                }
            }
            '7' => {
                // symbol above else to the right
                if position.0 > char_position.0 {
                    (char_position.0, char_position.1 - 1)
                } else {
                    (char_position.0 + 1, char_position.1)
                }
            }
            'F' => {
                // symbol above else to the left
                if position.0 > char_position.0 {
                    (char_position.0, char_position.1 + 1)
                } else {
                    (char_position.0 + 1, char_position.1)
                }
            }
            _ => unimplemented!()
        }
    }

    let mut steps: f64 = 0.0;
    let mut current = start.clone();
    let mut next = search_first_pipe(current);

    // search first pipe
    while lines[next.0][next.1] != 'S' {
        let temp_next = get_next(lines[next.0][next.1], current, next);
        current = next;
        next = temp_next;

        // max_distance = max(max_distance, start.0.abs_diff(current.0) + start.1.abs_diff(current.1))
        steps += 1.0;
    }

    let max_distance: u32 = (steps / 2.0).ceil() as u32;
    println!("The max distance is {max_distance}");
}
