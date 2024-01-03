use std::error::Error;
use std::fs;
use std::iter::zip;
use std::ops::RangeInclusive;

use aoc::Config;

pub fn run(config: Config) -> Result<(Option<u64>, Option<u64>), Box<dyn Error>> {
    let almanac = fs::read_to_string(config.file_path)?;

    let part_one = min_location(&almanac);
    let part_two = seeds_range_min_location(&almanac);

    Ok((Some(part_one), Some(part_two)))
}

pub fn min_location(almanac: &str) -> u64 {
    let mut lines = almanac.lines();
    let mut src_numbers = initial_seeds(&mut lines);
    let mut dst_number: Vec<Option<u64>> = vec![None; src_numbers.len()];

    // skips first seed-to-soil map
    for line in lines.filter(|l| !l.trim_start().is_empty()).skip(1) {
        if line.ends_with("map:") {
            src_numbers = collect_values(&src_numbers, dst_number.drain(..));
            dst_number.resize_with(src_numbers.len(), Default::default);
            continue;
        }

        let (dst_range, src_range, _) = ranges_from(line);
        for (i, src) in src_numbers.iter().enumerate() {
            if already_mapped(&dst_number, i) {
                continue;
            }

            if src_range.contains(src) {
                dst_number[i] = Some(dst_range.start() + (*src - src_range.start()));
            }
        }
    }

    collect_values(&src_numbers, dst_number.into_iter())
        .into_iter()
        .min()
        .unwrap()
}

fn already_mapped(dst: &Vec<Option<u64>>, i: usize) -> bool {
    matches!(dst[i], Some(_))
}

fn initial_seeds<'a>(it: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    it.next()
        .into_iter()
        .flat_map(|s| s.strip_prefix("seeds: "))
        .flat_map(|s| s.split_whitespace())
        .flat_map(|s| s.parse())
        .collect()
}

fn collect_values<'a>(src: &'a Vec<u64>, dst: impl Iterator<Item = Option<u64>>) -> Vec<u64> {
    dst.enumerate().map(|(i, v)| v.unwrap_or(src[i])).collect()
}

pub fn seeds_range_min_location(almanac: &str) -> u64 {
    let mut lines = almanac.lines();
    let mut source_ranges = initial_seed_ranges(&mut lines);
    let mut unmapped_ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut mapped_ranges: Vec<RangeInclusive<u64>> = vec![];

    // skips first seed-to-soil map
    for line in lines.filter(|l| !l.trim_start().is_empty()).skip(1) {
        if unmapped_ranges.len() > 0 {
            source_ranges.extend(unmapped_ranges.drain(..));
        }

        if line.ends_with("map:") {
            source_ranges.append(&mut mapped_ranges);
            continue;
        }

        let (map_dst_range, map_src_range, len) = ranges_from(line);

        for src_range in source_ranges.drain(..) {
            if !overlaps(&src_range, &map_src_range) {
                unmapped_ranges.push(src_range);
                continue; // keep non overlapping ranges untouched
            }

            let mut start = *map_dst_range.start();

            // source range start endpoint is whithin source range map
            if map_src_range.end() - src_range.start() <= len {
                start = map_dst_range.start() + (len - (map_src_range.end() - src_range.start())) - 1;
            } else {
                unmapped_ranges.push(*src_range.start()..=*map_src_range.start() - 1);
            }

            let mut end = *map_dst_range.end();

            // source range end endpoint is whithin source range map
            if src_range.end() - map_src_range.start() <= len {
                end = map_dst_range.start() + (src_range.end() - map_src_range.start());
            } else {
                unmapped_ranges.push(*map_src_range.end() + 1..=*src_range.end());
            }

            mapped_ranges.push(start..=end);
        }
    }

    mapped_ranges
        .iter()
        .chain(&unmapped_ranges)
        .map(|range| *range.start())
        .min()
        .unwrap()
}

fn initial_seed_ranges<'a>(it: &mut impl Iterator<Item = &'a str>) -> Vec<RangeInclusive<u64>> {
    let (seeds, lens): (Vec<_>, Vec<_>) = it
        .next()
        .into_iter()
        .flat_map(|s| s.strip_prefix("seeds: "))
        .flat_map(|s| s.split_whitespace())
        .flat_map(|s| s.parse::<u64>())
        .enumerate()
        .partition(|pair| pair.0 % 2 == 0);

    zip(
        seeds.into_iter().map(|pair| pair.1),
        lens.into_iter().map(|pair| pair.1),
    )
    .map(|(seed, len)| seed..=seed + len - 1)
    .collect()
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    b.start() <= a.end() && a.start() <= b.end()
}

fn ranges_from(s: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>, u64) {
    let range: Vec<u64> = s.split_whitespace().flat_map(|s| s.parse()).collect();
    (
        range[0]..=range[0] + range[2] - 1,
        range[1]..=range[1] + range[2] - 1,
        range[2],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let almanac = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, min_location(almanac));
    }

    #[test]
    fn part2() {
        let almanac = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(46, seeds_range_min_location(almanac));
    }
}
