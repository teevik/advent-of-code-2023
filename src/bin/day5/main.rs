#![feature(test)]
#![feature(iter_array_chunks)]

use std::ops::Range;

use itertools::Itertools;

// trait RangeExt {
//     fn intersect(self, other: &Range<i64>) -> Range<i64>;
// }
//
// impl RangeExt for Range<i64> {
//     fn intersect(self, other: Range<i64>) -> Range<i64> {
//         let a = self;
//         self
//     }
// }

#[derive(Clone, Copy, Debug)]
struct SeedRange {
    from: i64, // Inclusive
    to: i64,   // Exclusive
}

impl SeedRange {
    fn new(from: i64, length: i64) -> Self {
        let to = from + length;

        Self { from, to }
    }
}

#[derive(Debug)]
struct MapRange {
    source_start: i64,      // Inclusive
    source_end: i64,        // Exclusive
    destination_start: i64, // Inclusive
    destination_end: i64,   // Exclusive
}

impl MapRange {
    fn new(source_start: i64, destination_start: i64, length: i64) -> Self {
        let source_end = source_start + length;
        let destination_end = destination_start + length;

        Self {
            source_start,
            source_end,
            destination_start,
            destination_end,
        }
    }

    fn map(&self, number: i64) -> Option<i64> {
        let is_in_range = number >= self.source_start && number < self.source_end;

        is_in_range.then_some(number - self.source_start + self.destination_start)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, number: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(number))
            .unwrap_or(number)
    }
}

fn parse_map(input: &str) -> Map {
    let lines = input.lines().skip(1); // Skip label

    let ranges = lines
        .map(|line| {
            let numbers = line.split(' ').map(|number| number.parse::<i64>().unwrap());

            let (destination_start, source_start, length) = numbers.collect_tuple().unwrap();

            MapRange::new(source_start, destination_start, length)
        })
        .sorted_by(|a, b| a.source_start.cmp(&b.source_start))
        .collect_vec();

    Map { ranges }
}

fn part_1(input: &str) -> i64 {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = &seeds[7..]; // Skip "seeds: "

    let maps = maps.split("\n\n").map(parse_map).collect_vec();

    let mut seeds = seeds
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect_vec();

    for map in maps {
        for seed in &mut seeds {
            *seed = map.map(*seed);
        }
    }

    seeds.into_iter().min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let seeds = &seeds[7..]; // Skip "seeds: "

    let maps = maps.split("\n\n").map(parse_map).collect::<Vec<_>>();

    let seeds = seeds
        .split(' ')
        .map(|seed| seed.parse::<i64>().unwrap())
        .array_chunks()
        .map(|[from, length]| SeedRange::new(from, length))
        .collect::<Vec<_>>();

    // for map in maps.into_iter() {
    //     for seed in seeds.iter_mut().progress() {
    //         *seed = map.map(*seed);
    //     }
    // }
    // seeds.into_iter().min().unwrap()

    // let locations = seeds.into_par_iter().map(|seed| {
    //     let mut seed = seed;
    //
    //     for map in &maps {
    //         seed = map.map(seed);
    //     }
    //
    //     seed
    // });
    //
    // locations.progress().min().unwrap()

    let mut current_ranges = seeds;

    for map in maps {
        let mut new_ranges = Vec::new();

        for mut range in current_ranges.iter().copied() {
            for rule in &map.ranges {
                let offset = rule.destination_start - rule.source_start;

                let rule_applies = range.from <= range.to
                    && range.from <= rule.source_end
                    && range.to >= rule.source_start;

                if rule_applies {
                    if range.from < rule.source_start {
                        new_ranges.push(SeedRange {
                            from: range.from,
                            to: rule.source_start - 1,
                        });

                        range.from = rule.source_start;

                        if range.to < rule.source_end {
                            new_ranges.push(SeedRange {
                                from: range.from + offset,
                                to: range.to + offset,
                            });
                            range.from = range.to + 1;
                        } else {
                            new_ranges.push(SeedRange {
                                from: range.from + offset,
                                to: rule.source_end - 1 + offset,
                            });
                            range.from = rule.source_end;
                        }
                    } else if range.to < rule.source_end {
                        new_ranges.push(SeedRange {
                            from: range.from + offset,
                            to: range.to + offset,
                        });
                        range.from = range.to + 1;
                    } else {
                        new_ranges.push(SeedRange {
                            from: range.from + offset,
                            to: rule.source_end - 1 + offset,
                        });
                        range.from = rule.source_end;
                    }
                }
            }

            if range.from <= range.to {
                new_ranges.push(range);
            }
        }

        current_ranges = new_ranges;
    }

    current_ranges
        .into_iter()
        .map(|range| range.from)
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

    #[bench]
    fn bench_part_1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }
}
