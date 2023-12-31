use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<i32>, Option<i32>), Box<dyn Error>> {
    let oasis_report = fs::read_to_string(config.file_path)?;

    let part_one = sum_extrapolated_report(&oasis_report);
    let part_two = sum_backward_extrapolated_report(&oasis_report);

    Ok((Some(part_one), Some(part_two)))
}

pub fn sum_extrapolated_report(oasis_reports: &str) -> i32 {
    oasis_reports
        .lines()
        .map(|report| {
            let history = history_from(report);
            *history.last().unwrap() + calc_differences(&history)
        })
        .sum()
}

pub fn sum_backward_extrapolated_report(oasis_reports: &str) -> i32 {
    oasis_reports
        .lines()
        .map(|report| {
            let history = history_from(report);
            *history.first().unwrap() - cal_differences_backward(&history)
        })
        .sum()
}

fn history_from(s: &str) -> Vec<i32> {
    s.split_ascii_whitespace().flat_map(|n| n.parse()).collect()
}

fn calc_differences(history: &Vec<i32>) -> i32 {
    match finite_differences(&history) {
        Some(diffs) => *diffs.last().unwrap() + calc_differences(&diffs),
        None => 0,
    }
}

fn finite_differences(sequence: &Vec<i32>) -> Option<Vec<i32>> {
    let diffs: Vec<i32> = sequence.windows(2).map(|n| n[1] - n[0]).collect();
    diffs.iter().any(|n| n != &0).then_some(diffs).or(None)
}

fn cal_differences_backward(history: &Vec<i32>) -> i32 {
    match finite_differences(&history) {
        Some(diffs) => *diffs.first().unwrap() - cal_differences_backward(&diffs),
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let oasis_report = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(114, sum_extrapolated_report(oasis_report));
    }

    #[test]
    fn part2() {
        let oasis_report = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(2, sum_backward_extrapolated_report(oasis_report));
    }
}
