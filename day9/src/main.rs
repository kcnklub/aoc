use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in input.lines() {
        let values = line
            .split(" ")
            .map(|x| x.trim())
            .map(|x| x.parse::<i32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<i32>>();

        let mut sets = Vec::new();
        let mut copy = values.clone();

        let mut still_looking = true;
        while still_looking {
            sets.push(copy.clone());
            let mut differences = Vec::new();
            for i in 0..copy.len() - 1 {
                let current = copy[i];
                let next = copy[i + 1];

                differences.push(next - current);
            }

            copy = differences.clone();

            // Check if all differences are 0
            still_looking = differences.iter().any(|x| *x != 0);
            if !still_looking {
                sets.push(differences.clone());
            }
        }

        let mut sets_copy = sets.clone();
        sets_copy.reverse();
        for (index, set) in sets.iter_mut().rev().enumerate() {
            println!("index: {}, set: {:?}", index, set);
            if index == 0 {
                set.push(0);
                sets_copy[index].push(0);
            } else {
                let last = sets_copy[index - 1].last().unwrap();
                println!("last: {}", last);
                let curr_last = set.last().unwrap();
                println!("curr_last: {}", curr_last);
                let diff = *curr_last + *last;
                set.push(diff);
                sets_copy[index].push(diff);
            }
        }

        let last = sets_copy.last().unwrap().last().unwrap();

        sum += last;
    }

    println!("sum: {}", sum);
}
