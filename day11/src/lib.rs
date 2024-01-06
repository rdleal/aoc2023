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
    let (expansion_rate_row, expansion_rate_col) = expansion_rate_from(image, expansion_rate);

    let galaxies: Vec<(usize, usize)> = image
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, ch)| {
                    if ch != '#' {
                        return None;
                    }
                    Some((
                        expansion_rate_row[row] as usize + row,
                        expansion_rate_col[col] as usize + col,
                    ))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    unique_pairs(&galaxies)
        .into_iter()
        .map(manhatan_distance)
        .sum()
}

fn manhatan_distance(((x1, y1), (x2, y2)): ((usize, usize), (usize, usize))) -> u64 {
    ((x2 as i64 - x1 as i64).abs() + (y2 as i64 - y1 as i64).abs()) as u64
}

fn expansion_rate_from(image: &str, rate: u64) -> (Vec<u64>, Vec<u64>) {
    let universe_grid: Vec<Vec<char>> = image.lines().map(|u| u.chars().collect()).collect();
    let mut expansion_rate_row = vec![0; universe_grid.len()];
    let mut expansion_rate_col = vec![0; universe_grid[0].len()];

    for (row, line) in universe_grid.iter().enumerate() {
        if !line.contains(&'#') {
            expansion_rate_row[row] += rate - 1;
        }
        if row > 0 {
            expansion_rate_row[row] += expansion_rate_row[row - 1];
        }
    }

    for (col, _) in universe_grid[0].iter().enumerate() {
        if universe_grid.iter().all(|s| s[col] == '.') {
            expansion_rate_col[col] += rate - 1;
        }
        if col > 0 {
            expansion_rate_col[col] += expansion_rate_col[col - 1];
        }
    }

    (expansion_rate_row, expansion_rate_col)
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
