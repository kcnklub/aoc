fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let lines = file
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let time_line = lines[0];
    let times_string = time_line.split(":").collect::<Vec<&str>>();
    let times_string = times_string.get(1).unwrap();
    let times = times_string.split(" ").collect::<Vec<&str>>();
    let times = times.iter().map(|time| time.trim()).collect::<Vec<&str>>();

    let mut time = String::new();
    for t in times {
        time.push_str(t);
    }
    let time = time.parse::<u64>().unwrap();

    let distance_line = lines[1];
    let distances_line = distance_line.split(":").collect::<Vec<&str>>();
    let distances_string = distances_line.get(1).unwrap();
    let distances = distances_string.split(" ").collect::<Vec<&str>>();
    let distances = distances
        .iter()
        .map(|distance| distance.trim())
        .collect::<Vec<&str>>();

    let mut distance = String::new();
    for d in distances {
        distance.push_str(d);
    }

    let distance = distance.parse::<u64>().unwrap();

    let race = Race { time, distance };

    println!("Race: {:?}", race);

    let mut wins = 0;
    for i in 0..race.time {
        let speed = i; // mm/s
        let distance = (race.time - i) * speed;

        println!("Covered {}mm in {}s", distance, race.time - i);

        if distance > race.distance {
            println!("win with time held: {} to distance: {}", i, distance);
            wins += 1;
        }
    }

    println!("{}", wins);
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
