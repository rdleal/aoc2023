use std::collections::HashMap;
use std::error::Error;
use std::fs;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<usize>, Option<usize>), Box<dyn Error>> {
    let instructions = fs::read_to_string(config.file_path)?;

    let part_one = steps_to_zzz(&instructions);
    let part_two = simultaneous_steps_to_z(&instructions);

    Ok((Some(part_one), Some(part_two)))
}

pub fn steps_to_zzz(instructions: &str) -> usize {
    let mut lines = instructions.lines();
    let directions: Vec<char> = lines.next().iter().flat_map(|l| l.chars()).collect();

    // skips empty line
    lines.next();

    let network: HashMap<&str, (&str, &str)> = HashMap::from_iter(nodes_and_edges_from(lines));

    let mut next = "AAA";
    directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(step, dir)| {
            if next == "ZZZ" {
                return Some(step);
            }
            let edges = network.get(&next).expect("valid edges");
            next = node_from_direction(*dir, edges);
            None
        })
        .expect("valid step count")
}

pub fn simultaneous_steps_to_z(instructions: &str) -> usize {
    let mut lines = instructions.lines();
    let directions: Vec<_> = lines.next().iter().flat_map(|l| l.chars()).collect();

    // skips empty line
    lines.next();

    let network: HashMap<&str, (&str, &str)> = HashMap::from_iter(nodes_and_edges_from(lines));
    let initial_nodes = network.keys().filter(|n| n.ends_with("A")).cloned();

    let steps = initial_nodes
        .map(|mut next| {
            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(step, dir)| {
                    if next.ends_with("Z") {
                        return Some(step);
                    }
                    let edges = network.get(next).expect("valid edges");
                    next = node_from_direction(*dir, edges);
                    None
                })
                .expect("valid step count")
        })
        .collect::<Vec<usize>>();

    steps.into_iter().reduce(lcm).unwrap()
}

fn node_from_direction<'a>(dir: char, (left, right): &(&'a str, &'a str)) -> &'a str {
    match dir {
        'L' => left,
        'R' => right,
        _ => panic!("invalid direction"),
    }
}

fn nodes_and_edges_from<'a>(
    it: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = (&'a str, (&'a str, &'a str))> {
    it.map(|s| {
        let (node, raw_edges) = s.split_once(" = ").unwrap();
        let edges = raw_edges
            .trim_matches(|c| c == '(' || ')' == c)
            .split_once(", ")
            .unwrap();
        (node, edges)
    })
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let instructions = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, steps_to_zzz(instructions));
    }

    #[test]
    fn part1_another_sample() {
        let instructions = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, steps_to_zzz(instructions));
    }

    #[test]
    fn part2() {
        let instructions = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(6, simultaneous_steps_to_z(instructions));
    }
}
