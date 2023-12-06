use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let lines = input.lines();
    let lines_ref = lines.clone();
    let lines_ref = lines_ref.into_iter().collect::<Vec<&str>>(); // get a copy to reference while
                                                                  //
    let mut gear_location_map = HashMap::new();
    //
    let mut sum = 0;
    // looping over the original

    for (index, line) in lines.enumerate() {
        let mut last_found = 0;
        for (c_index, char) in line.chars().enumerate() {
            if char == '.' || c_index < last_found {
                continue;
            }
            if char.is_numeric() {
                let mut number_string = String::from(char);
                let should_scan = true;
                let mut current_index = c_index + 1;
                while should_scan {
                    let next_char = lines_ref[index].chars().nth(current_index);
                    if let Some(next_char) = next_char {
                        if next_char.is_numeric() {
                            number_string.push(next_char);
                            current_index += 1;
                        } else {
                            last_found = current_index;
                            break;
                        }
                    } else {
                        break;
                    }
                }

                // if we have the number we need to perform our checks for symbols

                let mut is_touching = false;

                // check current line
                if c_index > 0 {
                    let prev_char = lines_ref[index].chars().nth(c_index - 1);
                    if let Some(prev_char) = prev_char {
                        if symbol_is_touching(&prev_char) {
                            is_touching = true;
                        }
                    }
                }
                if current_index < lines_ref[index].len() {
                    let next_char = lines_ref[index].chars().nth(current_index);
                    if let Some(next_char) = next_char {
                        if symbol_is_touching(&next_char) {
                            is_touching = true;
                        }
                    }
                }

                // check previous line
                if index > 0 {
                    let prev_line = lines_ref[index - 1];
                    let prev_line = prev_line.chars().collect::<Vec<char>>();

                    let starting_index = if c_index > 0 { c_index - 1 } else { c_index };
                    let ending_index = if current_index < prev_line.len() {
                        current_index + 1
                    } else {
                        current_index
                    };
                    for i in starting_index..ending_index {
                        if symbol_is_touching(&prev_line[i]) {
                            is_touching = true;
                        }
                    }
                }

                if index < lines_ref.len() - 1 {
                    let next_line = lines_ref[index + 1];
                    let next_line = next_line.chars().collect::<Vec<char>>();

                    let starting_index = if c_index > 0 { c_index - 1 } else { c_index };
                    let ending_index = if current_index < next_line.len() {
                        current_index + 1
                    } else {
                        current_index
                    };
                    for i in starting_index..ending_index {
                        if symbol_is_touching(&next_line[i]) {
                            is_touching = true;
                        }
                    }
                }

                if is_touching {
                    //sum += number_string.parse::<i32>().unwrap();
                }

                // lets check for gears
                let mut gear_location = (0, 0);
                let mut is_gear = false;
                if c_index > 0 {
                    let prev_char = lines_ref[index].chars().nth(c_index - 1);
                    if let Some(prev_char) = prev_char {
                        if prev_char == '*' {
                            gear_location = (index, c_index - 1);
                            is_gear = true;
                        }
                    }
                }
                if current_index < lines_ref[index].len() {
                    let next_char = lines_ref[index].chars().nth(current_index);
                    if let Some(next_char) = next_char {
                        if next_char == '*' {
                            gear_location = (index, current_index);
                            is_gear = true;
                        }
                    }
                }

                // check previous line
                if index > 0 {
                    let prev_line = lines_ref[index - 1];
                    let prev_line = prev_line.chars().collect::<Vec<char>>();

                    let starting_index = if c_index > 0 { c_index - 1 } else { c_index };
                    let ending_index = if current_index < prev_line.len() {
                        current_index + 1
                    } else {
                        current_index
                    };
                    for i in starting_index..ending_index {
                        if prev_line[i] == '*' {
                            gear_location = (index - 1, i);
                            is_gear = true;
                        }
                    }
                }

                if index < lines_ref.len() - 1 {
                    let next_line = lines_ref[index + 1];
                    let next_line = next_line.chars().collect::<Vec<char>>();

                    let starting_index = if c_index > 0 { c_index - 1 } else { c_index };
                    let ending_index = if current_index < next_line.len() {
                        current_index + 1
                    } else {
                        current_index
                    };
                    for i in starting_index..ending_index {
                        if next_line[i] == '*' {
                            gear_location = (index + 1, i);
                            is_gear = true;
                        }
                    }
                }

                if is_gear {
                    println!(
                        "found a gear scanning at {:?}, with number {}",
                        (index, c_index),
                        number_string
                    );

                    if let Some((value, _location)) = gear_location_map.get(&gear_location) {
                        let parsed = number_string.parse::<i32>().unwrap();
                        let adding = parsed * value;
                        println!("Adding {} * {} = {}", parsed, value, adding);
                        sum += adding;
                        gear_location_map.remove(&gear_location);
                    } else {
                        let parsed = number_string.parse::<i32>().unwrap();
                        let number_info = (parsed, (index, c_index));
                        gear_location_map.insert(gear_location, number_info);
                    }
                    continue;
                }
            }
        }
    }

    println!("Gear map: {:?}", gear_location_map);
    let expected = 467835;
    println!("Sum: {}", sum);
    println!("Difference: {}", sum - expected);
}

fn symbol_is_touching(c: &char) -> bool {
    !c.is_alphanumeric() && *c != '.' && *c != '*'
}
