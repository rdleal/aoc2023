use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u64>, Option<u64>), Box<dyn Error>> {
    let image = fs::read_to_string(config.file_path)?;

    let part_one = sum_shortest_paths(&image, 2);
    let part_two = sum_shortest_paths(&image, 1_000_000);

    Ok((Some(part_one), Some(part_two)))
}

pub fn sum_shortest_paths(image: &str, expansion_rate: u64) -> u64 {
    let galaxies = galaxies_from(image, expansion_rate);

    unique_pairs(&galaxies)
        .into_iter()
        .map(manhattan_distance)
        .sum()
}

fn galaxies_from(image: &str, rate: u64) -> Vec<(usize, usize)> {
    let expansion_rate_row = image
        .lines()
        .scan(0, |expansion, line| {
            if !line.contains('#') {
                *expansion += rate - 1
            }
            Some(*expansion)
        })
        .collect::<Vec<u64>>();

    let universe_grid: Vec<Vec<char>> = image.lines().map(|u| u.chars().collect()).collect();
    let mut galaxies: Vec<(usize, usize)> = vec![];
    let mut expansion_rate_col = 0;
    for col in 0..universe_grid[0].len() {
        let mut found_galaxy = false;
        for row in 0..universe_grid.len() {
            if universe_grid[row][col] == '#' {
                found_galaxy = true;
                galaxies.push((
                    expansion_rate_row[row] as usize + row,
                    expansion_rate_col as usize + col,
                ));
            }
        }

        if !found_galaxy {
            expansion_rate_col += rate - 1;
        }
    }

    galaxies
}

fn unique_pairs(galaxies: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
    let mut results: Vec<((usize, usize), (usize, usize))> = vec![];
    let mut indices = [0, 1];
    let n = galaxies.len();

    while indices[0] <= n - 2 {
        results.push((galaxies[indices[0]], galaxies[indices[1]]));

        let mut i = 1isize;
        while i >= 0 && indices[i as usize] >= i as usize + n - 2 {
            i -= 1;
        }

        if i == -1 {
            break;
        }

        indices[i as usize] += 1;

        for j in i + 1..2 {
            indices[j as usize] = indices[j as usize - 1] + 1;
        }
    }

    results
}

fn manhattan_distance(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> u64 {
    ((x2 as i64 - x1 as i64).abs() + (y2 as i64 - y1 as i64).abs()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let image = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(374, sum_shortest_paths(image, 2));
    }

    #[test]
    fn part2_10x() {
        let image = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(1030, sum_shortest_paths(image, 10));
    }

    #[test]
    fn part2_100x() {
        let image = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(8410, sum_shortest_paths(image, 100));
    }
}
