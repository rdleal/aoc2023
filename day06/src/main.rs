use std::process;
use std::env;

use aoc::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    match day6::run(config) {
        Ok((part1, part2)) => println!("part 1: {:?}\npart 2: {:?}", part1, part2),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}

