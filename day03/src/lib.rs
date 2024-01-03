use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let engine = fs::read_to_string(config.file_path)?;

    let part_one = sum_parts(&engine);
    let part_two = sum_gears_ratio(&engine);

    Ok((Some(part_one), Some(part_two)))
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // top
    (-1, -1), // top left
    (-1, 1),  // top right
    (1, 0),   // bottom
    (1, -1),  // bottom left
    (1, 1),   // bottom right
    (0, -1),  // left
    (0, 1),   // right
];

fn within_boundaries(s: &Vec<&str>, row: isize, col: isize) -> bool {
    row >= 0 && row <= (s.len() - 1) as isize && col >= 0 && col <= (s[0].len() - 1) as isize
}

pub fn sum_parts(engine_schematics: &str) -> u32 {
    let s: Vec<&str> = engine_schematics.lines().collect();
    engine_schematics
        .lines()
        .map(|s| s.chars())
        .enumerate()
        .map(|(row, symbols)| {
            symbols
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '.' | '0'..='9' => 0,
                    _ => {
                        let mut visited = vec![vec![false; s[0].len()]; s.len()];
                        DIRECTIONS
                            .iter()
                            .map(|&(dir_row, dir_col)| {
                                (dir_row + row as isize, dir_col + col as isize)
                            })
                            .filter(|&(row, col)| within_boundaries(&s, row, col))
                            .filter_map(|(row, col)| {
                                search_num(&s, row as usize, col as usize, &mut visited)
                            })
                            .sum::<u32>()
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn sum_gears_ratio(engine_schematics: &str) -> u32 {
    let s: Vec<_> = engine_schematics.lines().collect();
    engine_schematics
        .lines()
        .map(|s| s.chars())
        .enumerate()
        .map(|(row, symbols)| {
            symbols
                .enumerate()
                .map(|(col, symbol)| match symbol {
                    '*' => {
                        let mut visited = vec![vec![false; s[0].len()]; s.len()];
                        let part_nums = DIRECTIONS
                            .iter()
                            .map(|&(dir_row, dir_col)| {
                                (dir_row + row as isize, dir_col + col as isize)
                            })
                            .filter(|&(row, col)| within_boundaries(&s, row, col))
                            .filter_map(|(row, col)| {
                                search_num(&s, row as usize, col as usize, &mut visited)
                            })
                            .collect::<Vec<u32>>();

                        if part_nums.len() == 2 {
                            return part_nums.iter().product();
                        }
                        0
                    }
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

fn search_num(s: &Vec<&str>, row: usize, col: usize, visited: &mut Vec<Vec<bool>>) -> Option<u32> {
    if visited[row][col] {
        return None;
    }

    visited[row][col] = true;
    match s[row].chars().nth(col) {
        Some(char @ '0'..='9') => {
            let mut num = char.to_digit(10);
            if col > 0 {
                num = part_num_left(&s, row, col - 1, visited)
                    .and_then(|n| Some(n * 10 + num.unwrap()))
                    .or(num);
            }

            if col < s[0].len() - 1 {
                num = part_num_right(&s, row, col + 1, num.unwrap(), visited).or(num);
            }

            num
        }
        _ => None,
    }
}

fn part_num_left(
    s: &Vec<&str>,
    row: usize,
    col: usize,
    visited: &mut Vec<Vec<bool>>,
) -> Option<u32> {
    visited[row][col] = true;
    match s[row].chars().nth(col) {
        Some(char @ '0'..='9') => {
            let num = char.to_digit(10);
            if col == 0 {
                return num;
            }
            part_num_left(&s, row, col - 1, visited)
                .and_then(|n| Some(n * 10 + num.unwrap()))
                .or(num)
        }
        _ => None,
    }
}

fn part_num_right(
    s: &Vec<&str>,
    row: usize,
    col: usize,
    prev: u32,
    visited: &mut Vec<Vec<bool>>,
) -> Option<u32> {
    visited[row][col] = true;
    match s[row].chars().nth(col) {
        Some(char @ '0'..='9') => {
            let n = prev * 10 + char.to_digit(10).unwrap();
            if col == s[0].len() - 1 {
                return Some(n);
            }
            part_num_right(&s, row, col + 1, n, visited).or(Some(n))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let engine_schematics = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, sum_parts(engine_schematics));
    }

    #[test]
    fn part2() {
        let engine_schematics = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(467835, sum_gears_ratio(engine_schematics));
    }

    #[test]
    fn part2_edge_case() {
        let engine_schematics = "\
407..114..
...*......
..35..633.
......#...
617*......
.....+.58.
...*604...
435.......
......755.
...$...*..
.664...590";

        assert_eq!(722435, sum_gears_ratio(engine_schematics));
    }
}
