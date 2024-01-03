use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u32>, Option<u32>), Box<dyn Error>> {
    let sketch = fs::read_to_string(config.file_path)?;

    let part_one = farthest_steps(&sketch);
    let part_two = count_enclosed_tiles(&sketch);

    Ok((Some(part_one), Some(part_two)))
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

pub fn farthest_steps(sketch: &str) -> u32 {
    let pipes_surface = sketch
        .lines()
        .map(|p| p.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start_position = s_coords(pipes_surface.iter());

    let start_pipe = look_around_from(start_position, &pipes_surface);

    let mut pipes = vec![start_pipe];
    let mut cycle_length = 1;

    while let Some((row, col, to_direction)) = pipes.pop() {
        if let Some(next) = next_pipe(pipes_surface[row][col], row, col, to_direction) {
            pipes.push(next);
        }
        cycle_length += 1;
    }

    cycle_length / 2
}

pub fn count_enclosed_tiles(sketch: &str) -> u32 {
    let pipes_surface = sketch
        .lines()
        .map(|p| p.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start_position = s_coords(pipes_surface.iter());

    let mut pipes_coords = vec![start_position];

    let mut pipes = vec![look_around_from(start_position, &pipes_surface)];

    while let Some((row, col, to_direction)) = pipes.pop() {
        pipes_coords.push((row, col));
        if let Some(next) = next_pipe(pipes_surface[row][col], row, col, to_direction) {
            pipes.push(next);
        }
    }

    // surveyor's formula to calculate area of polygons.
    let determinants = pipes_coords
        .windows(2)
        .map(|coords| {
            let (y1, x1) = coords[0];
            let (y2, x2) = coords[1];

            (x1 * y2) as i32 - (x2 * y1) as i32
        })
        .sum::<i32>()
        .abs() as u32;

    (determinants / 2) - (((pipes_coords.len() - 1) / 2) as u32) + 1
}

fn s_coords<'a>(surface: impl Iterator<Item = &'a Vec<char>>) -> (usize, usize) {
    surface
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .position(|col| col == &'S')
                .and_then(|j| Some((i, j)))
        })
        .expect("valid starting point S")
}

fn look_around_from(
    (s_row, s_col): (usize, usize),
    surface: &Vec<Vec<char>>,
) -> (usize, usize, Direction) {
    DIRECTIONS
        .iter()
        .find_map(|&dir| match dir {
            Direction::South if within_range(&surface, (s_row as i32 + 1, s_col as i32)) => {
                let (next_row, next_col) = (s_row + 1, s_col);
                match surface[next_row][next_col] {
                    '|' | 'L' | 'J' => Some((next_row, next_col, dir)),
                    _ => None,
                }
            }
            Direction::North if within_range(&surface, (s_row as i32 - 1, s_col as i32)) => {
                let (next_row, next_col) = (s_row - 1, s_col);
                match surface[next_row][next_col] {
                    '|' | '7' | 'F' => Some((next_row, next_col, dir)),
                    _ => None,
                }
            }
            Direction::East if within_range(&surface, (s_row as i32, s_col as i32 + 1)) => {
                let (next_row, next_col) = (s_row, s_col + 1);
                match surface[next_row][next_col] {
                    '-' | '7' | 'J' => Some((next_row, next_col, dir)),
                    _ => None,
                }
            }
            Direction::West if within_range(&surface, (s_row as i32, s_col as i32 - 1)) => {
                let (next_row, next_col) = (s_row, s_col - 1);
                match surface[next_row][next_col] {
                    '-' | 'L' | 'F' => Some((next_row, next_col, dir)),
                    _ => None,
                }
            }
            _ => None,
        })
        .expect("valid start pipe")
}

fn next_pipe(
    pipe: char,
    row: usize,
    col: usize,
    to_direction: Direction,
) -> Option<(usize, usize, Direction)> {
    if pipe == 'S' {
        return None;
    }
    let next = match (pipe, to_direction) {
        ('|', Direction::North) => (row - 1, col, Direction::North),
        ('|', Direction::South) => (row + 1, col, Direction::South),
        ('L', Direction::South) => (row, col + 1, Direction::West),
        ('L', Direction::East) => (row - 1, col, Direction::North),
        ('J', Direction::South) => (row, col - 1, Direction::East),
        ('J', Direction::West) => (row - 1, col, Direction::North),
        ('7', Direction::West) => (row + 1, col, Direction::South),
        ('7', Direction::North) => (row, col - 1, Direction::East),
        ('F', Direction::East) => (row + 1, col, Direction::South),
        ('F', Direction::North) => (row, col + 1, Direction::West),
        ('-', Direction::West) => (row, col + 1, Direction::West),
        ('-', Direction::East) => (row, col - 1, Direction::East),
        _ => unreachable!("should have a valid combination of pipe and direction"),
    };

    Some(next)
}

fn within_range(surface: &Vec<Vec<char>>, (row, col): (i32, i32)) -> bool {
    0 <= row && row < surface.len() as i32 && 0 <= col && col < surface[0].len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let sketch = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(8, farthest_steps(sketch));
    }

    #[test]
    fn part1_another_sample() {
        let sketch = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        assert_eq!(4, farthest_steps(sketch));
    }

    #[test]
    fn part2() {
        let sketch = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(4, count_enclosed_tiles(sketch));
    }

    #[test]
    fn part2_another_sampe() {
        let sketch = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, count_enclosed_tiles(sketch));
    }
}
