#![feature(test)]

use std::collections::{HashMap, HashSet};
use vek::Vec2;

#[derive(Debug)]
struct EngineNumber {
    number: i32,
    start_position: Vec2<i32>,
    horizontal_length: i32,
}

struct ParseResult {
    // All positions of a symbol
    symbol_positions: HashSet<Vec2<i32>>,

    // All positions of gears
    gear_positions: Vec<Vec2<i32>>,

    // All engine numbers
    engine_numbers: Vec<EngineNumber>,

    // Map from grid position to index of engine number
    engine_number_positions: HashMap<Vec2<i32>, usize>,
}

fn parse_input(input: &str) -> ParseResult {
    let mut symbol_positions = HashSet::<Vec2<i32>>::new();
    let mut gear_positions = Vec::<Vec2<i32>>::new();

    let mut engine_numbers = Vec::<EngineNumber>::new();
    let mut engine_number_positions = HashMap::<Vec2<i32>, usize>::new();

    for (y, line) in input.lines().enumerate() {
        let mut characters = line.bytes().enumerate().peekable();

        loop {
            match characters.next() {
                None => break,               // Done iterating
                Some((_, b'.')) => continue, // Ignore dots
                Some((x, char)) if char.is_ascii_digit() => {
                    let first_digit = char - b'0';

                    // let mut digits = vec![first_digit];
                    let mut sum = first_digit as i32;
                    let mut horizontal_length = 1;

                    while characters
                        .peek()
                        .is_some_and(|(_, char)| char.is_ascii_digit())
                    {
                        let next_digit = characters.next().expect("Checked before");
                        let next_digit = (next_digit.1 - b'0') as i32;

                        sum = (sum * 10) + next_digit;
                        horizontal_length += 1;
                    }

                    engine_numbers.push(EngineNumber {
                        number: sum,
                        start_position: Vec2::new(x as i32, y as i32),
                        horizontal_length,
                    });

                    let index = engine_numbers.len() - 1;

                    for x_offset in 0..horizontal_length {
                        engine_number_positions
                            .insert(Vec2::new(x as i32 + x_offset, y as i32), index);
                    }
                }
                Some((x, symbol)) => {
                    symbol_positions.insert(Vec2::new(x as i32, y as i32));

                    if symbol == b'*' {
                        gear_positions.push(Vec2::new(x as i32, y as i32));
                    }
                }
            }
        }
    }

    ParseResult {
        symbol_positions,
        gear_positions,
        engine_numbers,
        engine_number_positions,
    }
}

fn part_1(input: &str) -> i32 {
    let ParseResult {
        symbol_positions,
        engine_numbers,
        ..
    } = parse_input(input);

    let mut sum_of_part_numbers = 0;

    for EngineNumber {
        number,
        start_position,
        horizontal_length,
    } in engine_numbers
    {
        for y in (start_position.y - 1)..=(start_position.y + 1) {
            for x in (start_position.x - 1)..(start_position.x + horizontal_length + 1) {
                if y == start_position.y
                    && x >= start_position.x
                    && x < (start_position.x + horizontal_length)
                {
                    continue;
                }

                let position = Vec2::new(x, y);
                if symbol_positions.contains(&position) {
                    sum_of_part_numbers += number;
                }
            }
        }
    }

    sum_of_part_numbers
}

fn part_2(input: &str) -> i32 {
    let ParseResult {
        gear_positions,
        engine_numbers,
        engine_number_positions,
        ..
    } = parse_input(input);

    let gear_position_neighbors = gear_positions.into_iter().map(|gear_position| {
        let mut neighbors = Vec::new();

        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let target_position = gear_position + Vec2::new(x_offset, y_offset);

                if let Some(engine_number_index) = engine_number_positions.get(&target_position) {
                    if !neighbors.contains(engine_number_index) {
                        neighbors.push(*engine_number_index);
                    }
                }
            }
        }

        neighbors
    });

    let gear_ratios = gear_position_neighbors
        .filter(|neighbors| neighbors.len() == 2)
        .map(|neighbors| {
            neighbors
                .into_iter()
                .map(|engine_number_index| engine_numbers[engine_number_index].number)
                .product::<i32>()
        });

    gear_ratios.sum()
}

fn main() {
    let input = include_str!("input.txt");

    let part_1 = part_1(input);
    dbg!(part_1);

    let part_2 = part_2(input);
    dbg!(part_2);
}

#[cfg(test)]
mod test {
    extern crate test;
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 544433);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 76314915);
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
