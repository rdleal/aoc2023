use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();
            
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let calibrations = fs::read_to_string(config.file_path)?;

    let part_one = Some(sum(&calibrations));
    let part_two = Some(sum_with_spelled(&calibrations));

    Ok((part_one, part_two))
}

pub fn sum(calibrations: &str) -> u32 {
    calibrations
        .lines()
        .map(|c| find_calibration(c))
        .sum()
}

pub fn sum_with_spelled(calibrations: &str) -> u32 {
    calibrations
        .lines()
        .map(|c| unspell_digits(c))
        .map(|c| find_calibration(&c))
        .sum()
}

fn find_calibration(calibration: &str) -> u32 {
    let mut digits = calibration
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());

    match digits.next() {
        Some(first) => combine(first, digits.last()),
        None => 0,
    }
}

const SPELLED_DIGITS: [(&str, &str); 12] = [
    ("eightwo", "82"),
    ("twone", "21"),
    ("oneight", "18"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn unspell_digits(calibration: &str) -> String {
    let mut unspelled:String = String::from(calibration);
    for d in SPELLED_DIGITS {
        let (spelled, digit) = d;
        unspelled = unspelled.replace(spelled, digit);
    }
    unspelled
}

fn combine(first: u32, last: Option<u32>) -> u32 {
    match last {
        Some(n) => (first * 10) + n,
        None => (first * 10) + first,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        let calibrations = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, sum(calibrations));
    }

    #[test]
    fn day1_part2() {
        let calibrations = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, sum_with_spelled(calibrations));
    }
}

