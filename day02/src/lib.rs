use std::cmp;
use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let games = fs::read_to_string(config.file_path)?;

    let part_one = sum_ids(&games);
    let part_two = sum_powers(&games);

    Ok((Some(part_one), Some(part_two)))
}

const AVAILABLE_REDS: u32 = 12;
const AVAILABLE_GREENS: u32 = 13;
const AVAILABLE_BLUES: u32 = 14;

pub fn sum_ids(games: &str) -> u32 {
    games
        .lines()
        .map(|g| g.split_once(": ").unwrap())
        .filter(|(_, bag)| {
            bag.split("; ")
                .flat_map(|subset| subset.split(", "))
                .map(cube_ncolor)
                .all(|(n, color)| match color {
                    "red" => n <= AVAILABLE_REDS,
                    "green" => n <= AVAILABLE_GREENS,
                    "blue" => n <= AVAILABLE_BLUES,
                    _ => false,
                })
        })
        .map(|(game, _)| game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap())
        .sum()
}

fn cube_ncolor(cube: &str) -> (u32, &str) {
    cube.split_once(" ")
        .and_then(|(n, color)| Some((n.parse().unwrap(), color)))
        .unwrap()
}

pub fn sum_powers(games: &str) -> u32 {
    games
        .lines()
        .map(|game| {
            game.rsplit_once(": ")
                .unwrap()
                .1
                .split("; ")
                .flat_map(|subset| subset.split(", "))
        })
        .fold(0, |power, cubes| {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            for cube in cubes {
                let (n, color) = cube_ncolor(cube);
                match color {
                    "red" => red = cmp::max(n, red),
                    "green" => green = cmp::max(n, green),
                    "blue" => blue = cmp::max(n, blue),
                    _ => {}
                }
            }

            power + red * green * blue
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let games = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
Game 6: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 14 blue, 12 red";

        assert_eq!(14, sum_ids(games));
    }

    #[test]
    fn part2() {
        let games = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, sum_powers(games));
    }
}
