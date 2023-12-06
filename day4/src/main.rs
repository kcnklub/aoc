use std::{collections::HashMap, fs};

fn main() {
    let cards_string =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut sum = 0;

    let mut map = HashMap::new();

    for (index, card) in cards_string.lines().enumerate() {
        println!("\nCard: {}", index);
        compute_if_not_present(&mut map, index as u32, 1);
        let split = card.split(":").collect::<Vec<&str>>();
        let card = split[1].trim();
        let card = card.split("|").collect::<Vec<&str>>();
        let winning_numbers = card[0].trim();
        let winning_numbers = winning_numbers
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>())
            .filter_map(Result::ok)
            .collect::<Vec<u32>>();
        let numbers = card[1].trim();
        let numbers = numbers
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>())
            .filter_map(Result::ok)
            .collect::<Vec<u32>>();

        //1, 2, 4, 8, 16, 32, 64, 128, 256, 512
        let cards_won = get_number_of_matches(winning_numbers, numbers);
        println!("Cards won: {}", cards_won);

        if let Some(number_of_times_winning_this_card) = map.get(&(index as u32)) {
            let temp = *number_of_times_winning_this_card;
            println!("Number of times playing this card: {}", temp);
            let adding = temp * cards_won;

            for i in index + 1..cards_won as usize + index + 1 {
                compute_if_not_present(&mut map, i as u32, temp);
            }
            println!("Adding: {}", adding);
            sum += adding + 1;
        } else {
            sum += 1; // we haven't won this card before
            for i in index + 1..cards_won as usize + index + 1 {
                print!("{} ", i);
                compute_if_not_present(&mut map, i as u32, 1);
            }
        }
        println!("");
        println!("Sum: {}", sum);
        println!("");
    }

    println!("Map: {:?}", map);

    println!("Sum: {}", sum);
}

fn compute_if_not_present(map: &mut HashMap<u32, u32>, key: u32, value: u32) {
    if !map.contains_key(&key) {
        map.insert(key, 0);
    }
    let updated = map.get_mut(&key).unwrap();
    *updated += value;
}

fn get_number_of_matches(winning_numbers: Vec<u32>, numbers: Vec<u32>) -> u32 {
    let mut number_of_matches = 0;

    for winning_number in winning_numbers {
        if numbers.contains(&winning_number) {
            number_of_matches += 1;
        }
    }

    number_of_matches
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let winning_numbers = vec![1, 2, 3, 4];
        let numbers = vec![1, 2, 3, 4];
        let number_of_matches = super::get_number_of_matches(winning_numbers, numbers);
        assert_eq!(number_of_matches, 8);

        let winning_numbers = vec![1, 2];
        let numbers = vec![1, 2];
        let number_of_matches = super::get_number_of_matches(winning_numbers, numbers);
        assert_eq!(number_of_matches, 2);

        let winning_numbers = vec![1, 2];
        let numbers = vec![1, 2];
        let number_of_matches = super::get_number_of_matches(winning_numbers, numbers);
        assert_eq!(number_of_matches, 2);
    }
}
