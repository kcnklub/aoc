use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut ranked = vec![];
    for hand in input.lines() {
        let data = hand.split(" ").collect::<Vec<&str>>();
        let cards = data[0].to_string();
        let bet = data[1].parse::<u32>().unwrap();
        let hand = Hand::new(cards, bet);
        if ranked.len() == 0 {
            ranked.push(hand);
            continue;
        }

        let mut inserted = false;
        for (i, ranked_hand) in ranked.iter().enumerate() {
            println!("{:?}", hand);
            if hand.beats(ranked_hand) {
                ranked.insert(i, hand.clone());
                inserted = true;
                break;
            }
        }
        if !inserted {
            ranked.push(hand);
        }
    }

    let mut total = 0;
    let mut multiplier = 1;
    for hand in ranked.iter().rev() {
        total += hand.bet * multiplier;
        multiplier += 1;
    }

    println!("Total: {}", total);
}

#[derive(Debug, Clone)]
struct Hand {
    value: String,
    rank: HandRank,
    bet: u32,
}

impl Hand {
    fn new(value: String, bet: u32) -> Hand {
        let rank = determine_hand_rank(&value);
        Hand { value, bet, rank }
    }

    fn beats(&self, other: &Hand) -> bool {
        if self.rank == other.rank {
            for i in 0..5 {
                let self_card = self.value.chars().nth(i).unwrap();
                let self_value;
                if self_card.is_numeric() {
                    self_value = self_card.to_digit(10).unwrap();
                } else {
                    self_value = match self_card {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Something went wrong"),
                    }
                }
                let other_card = other.value.chars().nth(i).unwrap();
                let other_value;
                if other_card.is_numeric() {
                    other_value = other_card.to_digit(10).unwrap();
                } else {
                    other_value = match other_card {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Something went wrong"),
                    }
                }

                println!(
                    "Self:{}, {} Other:{} {}",
                    self_card, self_value, other_card, other_value
                );
                if self_value > other_value {
                    return true;
                } else if self_value < other_value {
                    return false;
                }
                println!("Tie");
            }
        }
        if self.rank == HandRank::FiveOfAKind {
            return true;
        }
        if self.rank == HandRank::FourOfAKind {
            if other.rank == HandRank::FiveOfAKind {
                return false;
            }
            return true;
        }
        if self.rank == HandRank::FullHouse {
            if other.rank == HandRank::FiveOfAKind || other.rank == HandRank::FourOfAKind {
                return false;
            }
            return true;
        }
        if self.rank == HandRank::ThreeOfAKind {
            if other.rank == HandRank::FiveOfAKind
                || other.rank == HandRank::FourOfAKind
                || other.rank == HandRank::FullHouse
            {
                return false;
            }
            return true;
        }
        if self.rank == HandRank::TwoPairs {
            if other.rank == HandRank::FiveOfAKind
                || other.rank == HandRank::FourOfAKind
                || other.rank == HandRank::FullHouse
                || other.rank == HandRank::ThreeOfAKind
            {
                return false;
            }
            return true;
        }
        if self.rank == HandRank::OnePair {
            if other.rank == HandRank::FiveOfAKind
                || other.rank == HandRank::FourOfAKind
                || other.rank == HandRank::FullHouse
                || other.rank == HandRank::ThreeOfAKind
                || other.rank == HandRank::TwoPairs
            {
                return false;
            }
            return true;
        }
        if self.rank == HandRank::HighCard {
            if other.rank == HandRank::FiveOfAKind
                || other.rank == HandRank::FourOfAKind
                || other.rank == HandRank::FullHouse
                || other.rank == HandRank::ThreeOfAKind
                || other.rank == HandRank::TwoPairs
                || other.rank == HandRank::OnePair
            {
                return false;
            }
            return true;
        }
        panic!("Something went wrong");
    }
}

fn determine_hand_rank(value: &str) -> HandRank {
    let mut card_map = HashMap::new();

    for card in value.split("") {
        if card == "" {
            continue;
        }
        let card = card.to_string();
        let count = card_map.entry(card).or_insert(0);
        *count += 1;
    }

    if card_map.len() == 1 {
        return HandRank::FiveOfAKind;
    }
    if card_map.len() == 2 {
        for val in card_map.values() {
            if *val == 4 {
                if let Some(_) = card_map.get("J") {
                    return HandRank::FiveOfAKind;
                }
                return HandRank::FourOfAKind;
            } else if *val == 3 {
                if let Some(_) = card_map.get("J") {
                    return HandRank::FiveOfAKind;
                }
                return HandRank::FullHouse;
            }
        }
    }

    if card_map.len() == 3 {
        for val in card_map.values() {
            if *val == 3 {
                if let Some(card) = card_map.get("J") {
                    if *card == 1 {
                        return HandRank::FourOfAKind;
                    }
                    if *card == 3 {
                        return HandRank::FourOfAKind;
                    }
                }
                return HandRank::ThreeOfAKind;
            } else if *val == 2 {
                if let Some(card) = card_map.get("J") {
                    if *card == 2 {
                        return HandRank::FourOfAKind;
                    }
                    if *card == 1 {
                        return HandRank::FullHouse;
                    }
                }
                return HandRank::TwoPairs;
            }
        }
    }
    if card_map.len() == 4 {
        if let Some(card) = card_map.get("J") {
            if *card == 1 {
                return HandRank::ThreeOfAKind;
            }
            if *card == 2 {
                return HandRank::ThreeOfAKind;
            }
        }
        return HandRank::OnePair;
    }
    if card_map.len() == 5 {
        if let Some(card) = card_map.get("J") {
            if *card == 1 {
                return HandRank::OnePair;
            }

        return HandRank::HighCard;
    }
    panic!("Something went wrong");
}

#[derive(Debug, PartialEq, Clone)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
