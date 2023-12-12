use rayon::prelude::*;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let data = input.split("\n\n").collect::<Vec<&str>>();

    let instructions = data[0];
    let instructions = instructions.chars();

    let instructions = instructions
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction"),
        })
        .collect::<Vec<Instruction>>();

    let mut parsed_map = HashMap::new();
    let mut starting_point = vec![];

    let map = data[1].split("\n").collect::<Vec<&str>>();
    for row in map {
        let row = row.split(" = ").collect::<Vec<&str>>();
        if row.len() != 2 {
            continue;
        }

        let start = row[0];
        if start.chars().nth(2).unwrap() == 'A' {
            starting_point.push(start);
        }

        let end = row[1];
        let end = end[1..end.len() - 1].split(", ").collect::<Vec<&str>>();
        let end = (end[0], end[1]);
        parsed_map.insert(start, end);
    }
    let mut current_locations = starting_point;
    let mut found_z_in_steps: [u128; 6] = [0; 6];
    let mut output = 1;
    let mut steps = 0;
    while should_move(&current_locations) {
        let instruction = &instructions[steps % &instructions.len()];
        current_locations = current_locations
            .par_iter()
            .map(|location| match instruction {
                Instruction::Left => parsed_map.get(location).unwrap().0,
                Instruction::Right => parsed_map.get(location).unwrap().1,
            })
            .collect::<Vec<&str>>();

        for (i, location) in current_locations.iter().enumerate() {
            if location.chars().nth(2).unwrap() == 'Z' && found_z_in_steps[i] == 0 {
                if found_z_in_steps[i] == 0 {
                    found_z_in_steps[i] = steps as u128;
                }
            }
        }

        steps += 1;
    }

    println!("Current locations: {:?}", current_locations);
    println!("Steps: {}", steps);
}

fn should_move(current_locations: &Vec<&str>) -> bool {
    current_locations
        .par_iter()
        .map(|location| {
            if location.chars().nth(2).unwrap() == 'Z' {
                return false;
            }
            true
        })
        .any(|x| x)
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}
