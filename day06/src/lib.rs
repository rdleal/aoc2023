use std::error::Error;
use std::fs;
use std::iter::zip;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let sheet = fs::read_to_string(config.file_path)?;

    let part_one = error_margin(&sheet);
    let part_two = big_error_margin(&sheet);

    Ok((Some(part_one), Some(part_two)))
}

pub fn error_margin(sheet: &str) -> u32 {
    list_races(sheet.lines())
        .iter()
        .map(|(max_time, max_dist)| {
            (1..*max_time).fold(0, |acc, i| {
                if i * (*max_time - i) <= *max_dist {
                    return acc;
                }
                acc + 1
            })
        })
        .product()
}

fn list_races<'a>(mut it: impl Iterator<Item = &'a str>) -> Vec<(u32, u32)> {
    zip(
        strip_and_parse("Time:", &mut it),
        strip_and_parse("Distance:", &mut it),
    )
    .collect()
}

fn strip_and_parse<'a>(prefix: &str, mut from: impl Iterator<Item = &'a str>) -> Vec<u32> {
    from.next()
        .iter()
        .flat_map(|s| s.strip_prefix(prefix))
        .flat_map(|s| s.split_whitespace())
        .flat_map(|s| s.parse())
        .collect::<Vec<u32>>()
}

pub fn big_error_margin(sheet: &str) -> u32 {
    let (max_time, max_dist) = race_time_dist(sheet.lines());
    (1..max_time).fold(0, |acc, i| {
        if i * (max_time - i) <= max_dist {
            return acc;
        }
        acc + 1
    })
}

fn race_time_dist<'a>(mut it: impl Iterator<Item = &'a str>) -> (u64, u64) {
    (
        strip_parse_one("Time:", &mut it),
        strip_parse_one("Distance:", &mut it),
    )
}

fn strip_parse_one<'a>(prefix: &str, mut from: impl Iterator<Item = &'a str>) -> u64 {
    from.next()
        .iter()
        .flat_map(|s| s.strip_prefix(prefix))
        .flat_map(|s| s.split_whitespace())
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sheet = "\
Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(288, error_margin(sheet));
    }

    #[test]
    fn part2() {
        let sheet = "\
Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(71503, big_error_margin(sheet));
    }
}
