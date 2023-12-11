#![feature(test)]

use itertools::Itertools;
use std::collections::HashSet;
use vek::Vec2;

fn find_galaxies(input: &str, expansion: usize) -> Vec<Vec2<usize>> {
    let map = input.lines().map(|line| line.as_bytes()).collect_vec();
    let height = map.len();
    let width = map[0].len();

    // Find empty columns and rows
    let map = &map;
    let columns = (0..width).map(move |x| (0..height).map(move |y| map[y][x]));
    let rows = map.iter().map(|line| line.iter());

    let empty_columns = columns
        .enumerate()
        .filter_map(|(x, mut chars)| chars.all(|char| char == b'.').then_some(x))
        .collect::<HashSet<_>>();

    let empty_rows = rows
        .enumerate()
        .filter_map(|(y, mut chars)| chars.all(|&char| char == b'.').then_some(y))
        .collect::<HashSet<_>>();

    // Find galaxy positions after expansion
    let mut galaxies = Vec::new();
    let mut y_expansion = 0;

    for y in 0..height {
        if empty_rows.contains(&y) {
            y_expansion += expansion;
        }

        let mut x_expansion = 0;

        for x in 0..width {
            if empty_columns.contains(&x) {
                x_expansion += expansion;
            }

            if map[y][x] == b'#' {
                galaxies.push(Vec2::new(x + x_expansion, y + y_expansion));
            }
        }
    }

    galaxies
}

fn find_distances(galaxies: Vec<Vec2<usize>>) -> usize {
    let mut total_distance = 0;

    for (from, to) in galaxies.into_iter().tuple_combinations() {
        let distance = from.zip(to).map(|(from, to)| from.abs_diff(to)).sum();

        total_distance += distance;
    }

    total_distance
}

fn part_1(input: &str) -> usize {
    let galaxies = find_galaxies(input, 1);

    find_distances(galaxies)
}

fn part_2(input: &str) -> usize {
    let galaxies = find_galaxies(input, 999_999);

    find_distances(galaxies)
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
    fn test_part1() {
        let result = part_1(INPUT);
        assert_eq!(result, 9605127);
    }

    #[test]
    fn test_part2() {
        let result = part_2(INPUT);
        assert_eq!(result, 458191688761);
    }

    #[bench]
    fn bench_part1(b: &mut test::Bencher) {
        b.iter(|| part_1(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut test::Bencher) {
        b.iter(|| part_2(INPUT));
    }
}
