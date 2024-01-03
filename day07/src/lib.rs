use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::iter::zip;

use aoc::Config;

#[derive(Debug, Copy, Clone)]
pub enum Joker {
    Card(char),
    None,
}

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let list_hands = fs::read_to_string(config.file_path)?;

    let part_one = total_winnings(&list_hands, Joker::None);
    let part_two = total_winnings(&list_hands, Joker::Card('J'));

    Ok((Some(part_one), Some(part_two)))
}

pub fn total_winnings(list_hands: &str, joker_card: Joker) -> u32 {
    let mut hands = hands_with_bids(list_hands.lines());
    hands.sort_unstable_by(|(hand_a, _), (hand_b, _)| {
        let mut order = hand_strength_by_type(hand_a, joker_card)
            .cmp(&hand_strength_by_type(hand_b, joker_card));
        if order == Ordering::Equal {
            order = hand_order_by_label(zip(hand_a.chars(), hand_b.chars()), joker_card);
        }
        order
    });

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| bid * (rank + 1) as u32)
        .sum()
}

fn hands_with_bids<'a>(list: impl Iterator<Item = &'a str>) -> Vec<(&'a str, u32)> {
    list.flat_map(|s| s.split_once(" "))
        .map(|(hand, bid)| (hand, bid.parse().expect("valid bid number")))
        .collect()
}

fn hand_strength_by_type(hand: &str, joker_card: Joker) -> u32 {
    let mut cards_count = HashMap::new();

    for c in hand.chars() {
        cards_count
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let joker = match joker_card {
        Joker::Card(label) => cards_count.remove(&label).unwrap_or(0),
        Joker::None => 0,
    };

    match hand.len() - cards_count.len() {
        // five of a kind
        4 | 5 => 1 + hand.len() as u32,
        // four of a kind or full house
        2 => cards_count.into_values().max().unwrap() + joker,
        //  three of a kind or two pair
        3 => cards_count.into_values().max().unwrap() + joker + 1,
        diff => diff as u32,
    }
}

fn hand_order_by_label(cards: impl Iterator<Item = (char, char)>, joker_card: Joker) -> Ordering {
    let mut order = Ordering::Equal;
    for (label_a, label_b) in cards {
        order = label_strength(&label_a, joker_card).cmp(&label_strength(&label_b, joker_card));
        if order != Ordering::Equal {
            break;
        }
    }

    order
}

fn label_strength<'a>(label: &'a char, joker_card: Joker) -> u32 {
    if let Joker::Card(joker_label) = joker_card {
        if joker_label == *label {
            return 1;
        }
    }
    match label {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        strength => strength.to_digit(10).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let list_hands = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(6440, total_winnings(list_hands, Joker::None));
    }

    #[test]
    fn part2() {
        let list_hands = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(5905, total_winnings(list_hands, Joker::Card('J')));
    }
}
