use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut starting_position = (0, 0);
    for i in 0..height {
        for j in 0..width {
            let char = input.lines().nth(i).unwrap().chars().nth(j).unwrap();
            if char == 'S' {
                starting_position = (i, j);
            }

            print!("{}", input.lines().nth(i).unwrap().chars().nth(j).unwrap());
        }
        println!();
    }

    let mut mapped_positions = vec![];
    for _i in 0..height {
        let mut row = vec![];
        for _j in 0..width {
            row.push(".".to_string());
        }
        mapped_positions.push(row);
    }

    let mut positions = vec![];
    let mut visited = vec![];

    let mut steps = 0;

    mapped_positions[starting_position.0][starting_position.1] = "0".to_string();
    positions.push(starting_position);

    while !positions.is_empty() {
        let current_positions = positions;
        positions = vec![];
        println!();
        println!("steps: {}", steps);
        println!("current_positions: {:?}", current_positions);
        for current_position in current_positions {
            mapped_positions[current_position.0][current_position.1] = steps.to_string();

            if visited.contains(&current_position) {
                continue;
            }
            visited.push(current_position);

            let current_char = input
                .lines()
                .nth(current_position.0)
                .unwrap()
                .chars()
                .nth(current_position.1)
                .unwrap();

            println!("current_position: {:?}", current_position);
            println!("current_char: {}", current_char);
            match current_char {
                'S' => {
                    if let Some(up) = scan_up(&current_position, &input) {
                        if !visited.contains(&up) {
                            positions.push(up);
                        }
                    }
                    if let Some(down) = scan_down(&current_position, &input) {
                        if !visited.contains(&down) {
                            positions.push(down);
                        }
                    }
                    if let Some(left) = scan_left(&current_position, &input) {
                        if !visited.contains(&left) {
                            positions.push(left);
                        }
                    }
                    if let Some(right) = scan_right(&current_position, &input) {
                        if !visited.contains(&right) {
                            positions.push(right);
                        }
                    }
                }
                '|' => {
                    if let Some(up) = scan_up(&current_position, &input) {
                        if !visited.contains(&up) {
                            positions.push(up);
                        }
                    }
                    if let Some(down) = scan_down(&current_position, &input) {
                        if !visited.contains(&down) {
                            positions.push(down);
                        }
                    }
                }
                '-' => {
                    if let Some(left) = scan_left(&current_position, &input) {
                        if !visited.contains(&left) {
                            positions.push(left);
                        }
                    }
                    if let Some(right) = scan_right(&current_position, &input) {
                        if !visited.contains(&right) {
                            positions.push(right);
                        }
                    }
                }
                'L' => {
                    if let Some(up) = scan_up(&current_position, &input) {
                        if !visited.contains(&up) {
                            positions.push(up);
                        }
                    }
                    if let Some(right) = scan_right(&current_position, &input) {
                        if !visited.contains(&right) {
                            positions.push(right);
                        }
                    }
                }
                'J' => {
                    if let Some(up) = scan_up(&current_position, &input) {
                        if !visited.contains(&up) {
                            positions.push(up);
                        }
                    }
                    if let Some(left) = scan_left(&current_position, &input) {
                        if !visited.contains(&left) {
                            positions.push(left);
                        }
                    }
                }
                'F' => {
                    if let Some(down) = scan_down(&current_position, &input) {
                        if !visited.contains(&down) {
                            positions.push(down);
                        }
                    }
                    if let Some(right) = scan_right(&current_position, &input) {
                        if !visited.contains(&right) {
                            positions.push(right);
                        }
                    }
                }
                '7' => {
                    if let Some(down) = scan_down(&current_position, &input) {
                        if !visited.contains(&down) {
                            positions.push(down);
                        }
                    }
                    if let Some(left) = scan_left(&current_position, &input) {
                        if !visited.contains(&left) {
                            positions.push(left);
                        }
                    }
                }
                _ => {}
            }
        }
        steps += 1;
    }
    println!("Steps: {}", steps);
    println!("Output");
    for row in mapped_positions {
        println!(
            "{:?}",
            row.iter()
                .map(|x| {
                    let mut x = x.to_string();
                    x.push(',');
                    x
                })
                .collect::<String>()
        );
    }
}

fn scan_up(starting_position: &(usize, usize), input: &String) -> Option<(usize, usize)> {
    if starting_position.0 as i32 - 1 >= 0 {
        let up = (starting_position.0 - 1, starting_position.1);
        let up_char = input.lines().nth(up.0).unwrap().chars().nth(up.1).unwrap();
        if up_char == 'F' || up_char == '7' || up_char == '|' {
            return Some(up);
        }
    }
    None
}

fn scan_down(starting_position: &(usize, usize), input: &String) -> Option<(usize, usize)> {
    if starting_position.0 + 1 < input.lines().count() {
        let down = (starting_position.0 + 1, starting_position.1);
        let down_char = input
            .lines()
            .nth(down.0)
            .unwrap()
            .chars()
            .nth(down.1)
            .unwrap();
        if down_char == 'J' || down_char == 'L' || down_char == '|' {
            return Some(down);
        }
    }
    None
}

fn scan_left(starting_position: &(usize, usize), input: &String) -> Option<(usize, usize)> {
    if starting_position.1 as i32 - 1 >= 0 {
        let left = (starting_position.0, starting_position.1 - 1);
        let left_char = input
            .lines()
            .nth(left.0)
            .unwrap()
            .chars()
            .nth(left.1)
            .unwrap();
        if left_char == 'F' || left_char == 'L' || left_char == '-' {
            println!("left: {:?}", left_char);
            return Some(left);
        }
    }
    None
}

fn scan_right(starting_position: &(usize, usize), input: &String) -> Option<(usize, usize)> {
    if starting_position.1 + 1 < input.lines().next().unwrap().chars().count() {
        let right = (starting_position.0, starting_position.1 + 1);
        let right_char = input
            .lines()
            .nth(right.0)
            .unwrap()
            .chars()
            .nth(right.1)
            .unwrap();
        if right_char == 'J' || right_char == '7' || right_char == '-' {
            return Some(right);
        }
    }
    None
}
