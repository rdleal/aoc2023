use std::collections::HashSet;
use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let scratchcards = fs::read_to_string(config.file_path)?;

    let part_one = sum_winning_numbers(&scratchcards);
    let part_two = count_cards(&scratchcards);

    Ok((Some(part_one), Some(part_two)))
}

pub fn sum_winning_numbers(scratchcards: &str) -> u32 {
    scratchcards
        .lines()
        .map(|card| {
            winning_numbers(card)
                .iter()
                .fold(0, |point, _| match point {
                    0 => 1,
                    _ => point * 2,
                })
        })
        .sum()
}

pub fn count_cards(scratchcards: &str) -> u32 {
    let cards: Vec<&str> = scratchcards.lines().collect();
    let mut memo: Vec<Option<u32>> = vec![None; cards.len()];
    scratchcards
        .lines()
        .enumerate()
        .map(|(i, _)| count_cards_stack(&cards, i, &mut memo))
        .sum()
}

fn count_cards_stack(cards: &Vec<&str>, start_at: usize, memo: &mut Vec<Option<u32>>) -> u32 {
    if let Some(n) = memo[start_at] {
        return n;
    }
    let count = 1 + winning_numbers(&cards[start_at])
        .iter()
        .enumerate()
        .map(|(i, _)| count_cards_stack(&cards, start_at + i + 1, memo))
        .sum::<u32>();

    memo[start_at] = Some(count);

    count
}

fn winning_numbers(s: &str) -> Vec<u32> {
    let (winning_points, my_points) =
        points_sets(s.split_once(": ").unwrap().1.split_once(" | ").unwrap());
    winning_points
        .intersection(&my_points)
        .cloned()
        .collect::<Vec<_>>()
}

fn points_sets((winning_points, my_points): (&str, &str)) -> (HashSet<u32>, HashSet<u32>) {
    (
        parse_point(winning_points),
        parse_point(my_points)
    )
}

fn parse_point(s: &str) -> HashSet<u32> {
    s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<HashSet<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let scratchcards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, sum_winning_numbers(scratchcards));
    }

    #[test]
    fn part2() {
        let scratchcards = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(30, count_cards(scratchcards));
    }
}
