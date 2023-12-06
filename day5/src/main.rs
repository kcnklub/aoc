use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let starting_values = sections[0];
    let starting_values = starting_values.split(":").collect::<Vec<&str>>()[1];
    let starting_values = starting_values.trim();
    let starting_values = starting_values
        .split(" ")
        .map(|x| x.trim())
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let copy = starting_values.clone();
    let mut computed_starting_values = vec![];

    println!("Computing starting values");
    for (index, val) in starting_values.iter().enumerate().step_by(2) {
        println!("index: {}, val: {}", index, val);
        let index = index + 1;
        let range = copy[index];
        for i in *val..*val + range {
            computed_starting_values.push(i);
        }
    }

    let seed_to_soil = sections[1];
    let seed_to_soil = seed_to_soil.split("\n").collect::<Vec<&str>>();
    let seed_to_soil = parse_map(&seed_to_soil);
    println!("Seed to soil done");

    let soil_to_fertilizer = sections[2];
    let soil_to_fertilizer = soil_to_fertilizer.split("\n").collect::<Vec<&str>>();
    let soil_to_fertilizer = parse_map(&soil_to_fertilizer);
    println!("Soil to fertilizer done");

    let fertilizer_to_water = sections[3];
    let fertilizer_to_water = fertilizer_to_water.split("\n").collect::<Vec<&str>>();
    let fertilizer_to_water = parse_map(&fertilizer_to_water);
    println!("Fertilizer to water done");

    let water_to_light = sections[4];
    let water_to_light = water_to_light.split("\n").collect::<Vec<&str>>();
    let water_to_light = parse_map(&water_to_light);
    println!("Water to light done");

    let light_to_temperature = sections[5];
    let light_to_temperature = light_to_temperature.split("\n").collect::<Vec<&str>>();
    let light_to_temperature = parse_map(&light_to_temperature);
    println!("Light to temperature done");

    let temperature_to_humidity = sections[6];
    let temperature_to_humidity = temperature_to_humidity.split("\n").collect::<Vec<&str>>();
    let temperature_to_humidity = parse_map(&temperature_to_humidity);
    println!("Temperature to humidity done");

    let humidity_to_location = sections[7];
    let humidity_to_location = humidity_to_location.split("\n").collect::<Vec<&str>>();
    let humidity_to_location = parse_map(&humidity_to_location);
    println!("Humidity to location done");

    println!("Mapping Started");
    let mapped_values = computed_starting_values
        .par_iter()
        .map(|x| map_value(*x, &seed_to_soil, "seed_to_soil"))
        .map(|x| map_value(x, &soil_to_fertilizer, "soil_to_fertilizer"))
        .map(|x| map_value(x, &fertilizer_to_water, "fertilizer_to_water"))
        .map(|x| map_value(x, &water_to_light, "water_to_light"))
        .map(|x| map_value(x, &light_to_temperature, "light_to_temperature"))
        .map(|x| map_value(x, &temperature_to_humidity, "temperature_to_humidity"))
        .map(|x| map_value(x, &humidity_to_location, "humidity_to_location"))
        .collect::<Vec<i128>>();

    println!("smallest: {}", mapped_values.iter().min().unwrap());
}

fn parse_map(map_data: &Vec<&str>) -> Vec<(i128, i128, i128)> {
    let mut map = vec![];
    for line in map_data.iter() {
        if line.contains("map") || line == &"" {
            continue;
        }
        let line = line.trim();

        let line = line
            .split(" ")
            .map(|x| x.trim())
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        let dest = line[0];
        let src = line[1];
        let range = line[2];

        map.push((dest, src, range));
    }
    map
}

fn map_value(val: i128, seed_to_soil: &Vec<(i128, i128, i128)>, message: &str) -> i128 {
    let mut mapped_value = val;
    for map in seed_to_soil.iter() {
        let diff = map.0 - map.1;

        if val >= map.1 && val < map.1 + map.2 {
            mapped_value = val + diff;
        }
    }
    mapped_value
}
