#![feature(test)]
#![feature(iter_array_chunks)]

use advent_of_code_2023::RangeExt;
use itertools::Itertools;
use rangemap::RangeMap;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    // Map from source range to destination offset
    rules: RangeMap<i64, i64>,
}

impl Map {
    fn find_destination_number(&self, source_number: i64) -> i64 {
        let offset = self.rules.get(&source_number);

        offset
            .map(|offset| source_number + offset) // Apply offset if a mapping exists
            .unwrap_or(source_number) // Otherwise source number
    }

    fn find_destination_ranges<'a>(
        &'a self,
        range: &'a Range<i64>,
    ) -> impl Iterator<Item = Range<i64>> + 'a {
        self.rules
            .overlapping(range)
            .map(|(source_range, destination_offset)| {
                let intersection = source_range.intersect(range);

                (intersection.start + destination_offset)..(intersection.end + destination_offset)
            })
    }
}

fn parse_map(input: &str) -> Map {
    let lines = input.lines().skip(1); // Skip label

    let rules = lines
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap());

            let (destination_start, source_start, length) = numbers.collect_tuple().unwrap();
            let source_end = source_start + length;
            let offset = destination_start - source_start;

            (source_start..source_end, offset)
        })
        .collect();

    Map { rules }
}

struct ParseResult<Seeds: Iterator<Item = i64>> {
    seeds: Seeds,
    maps: Vec<Map>,
}

fn parse_input(input: &str) -> ParseResult<impl Iterator<Item = i64> + '_> {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = seeds.strip_prefix("seeds: ").unwrap();

    let seeds = seeds
        .split_ascii_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap());
    let maps = maps.split("\n\n").map(parse_map).collect_vec();

    ParseResult { seeds, maps }
}

fn part_1(input: &str) -> i64 {
    let ParseResult { seeds, maps } = parse_input(input);

    let mut seeds = seeds.collect_vec();

    for map in maps {
        for seed in &mut seeds {
            *seed = map.find_destination_number(*seed);
        }
    }

    seeds.into_iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let ParseResult { seeds, maps } = parse_input(input);

    // Create seed ranges
    let seeds = seeds
        .array_chunks()
        .map(|[from, length]| from..(from + length))
        .collect_vec();

    let mut current_ranges = seeds;

    for map in maps {
        let new_ranges = current_ranges
            .iter()
            .flat_map(|range| map.find_destination_ranges(range))
            .collect_vec();

        current_ranges = new_ranges;
    }

    current_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    let part_1 = part_1(input);
    dbg!(part_1);

    let part_2 = part_2(input);
    dbg!(part_2);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 57075758);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 31161857);
    }

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
