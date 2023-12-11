#![feature(test)]

use ndarray::{Array, Array2};
use vek::Vec2;

struct ParseResult {
    map: Array2<char>,
    size: Vec2<usize>,
    start_point: Vec2<usize>,
}

fn print_map(map: &Array2<char>) {
    for row in map.columns() {
        for char in row {
            print!("{char}");
        }
        println!();
    }
}

fn parse_input(input: &str) -> ParseResult {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let lines = input.lines().enumerate();
    let chars = lines.flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, char)| ((x, y), char))
    });

    let mut map = Array::from_elem((width, height), '.');
    let mut start_point = Vec2::default();

    for ((x, y), char) in chars {
        let char = match char {
            'S' => {
                start_point = Vec2::new(x, y);
                'S'
            }
            '.' => '.',
            'L' => '╚',
            'J' => '╝',
            '7' => '╗',
            'F' => '╔',
            '|' => '║',
            '-' => '═',
            _ => unreachable!(),
        };

        map[(x, y)] = char;
    }

    let size = Vec2::new(width, height);

    ParseResult {
        map,
        size,
        start_point,
    }
}

fn find_next_pipe(
    map: &Array2<char>,
    current_position: Vec2<usize>,
    previous_position: Vec2<usize>,
) -> Option<Vec2<usize>> {
    let me = map.get(current_position.into_tuple()).unwrap();
    let neighbor_offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let next_pipe = neighbor_offsets
        .into_iter()
        .filter_map(|offset| {
            let offset: Vec2<i32> = offset.into();
            let neighbor_position = current_position.as_::<i32>() + offset;

            let is_in_bounds = neighbor_position.x >= 0
                && neighbor_position.y >= 0
                && neighbor_position.x < map.dim().0 as i32
                && neighbor_position.y < map.dim().1 as i32;

            if !is_in_bounds || neighbor_position == previous_position.as_::<i32>() {
                return None;
            }

            Some((neighbor_position.as_::<usize>(), offset))
        })
        .find_map(|(neighbor_position, offset)| {
            let neighbor = map.get(neighbor_position.into_tuple()).unwrap();

            let is_connected = match (me, offset.into_array(), neighbor) {
                ('S' | '═' | '╗' | '╝', [-1, 0], '═' | '╚' | '╔' | 'S') => true, // left
                ('S' | '═' | '╚' | '╔', [1, 0], '═' | '╗' | '╝' | 'S') => true,  // right
                ('S' | '║' | '╝' | '╚', [0, -1], '║' | '╔' | '╗' | 'S') => true, // top
                ('S' | '║' | '╔' | '╗', [0, 1], '║' | '╝' | '╚' | 'S') => true,  // bottom
                _ => false,
            };

            is_connected.then_some(neighbor_position)
        });

    next_pipe
}

fn part_1(input: &str) -> usize {
    let ParseResult {
        map, start_point, ..
    } = parse_input(input);

    let mut previous_position = start_point;
    let mut current_position = start_point;
    let mut total_steps = 0;

    loop {
        let next_position = find_next_pipe(&map, current_position, previous_position).unwrap();
        total_steps += 1;

        if next_position == start_point {
            break;
        }

        previous_position = current_position;
        current_position = next_position;
    }

    let loop_halfway_point = total_steps / 2;

    loop_halfway_point
}

fn part_2(input: &str) -> usize {
    let ParseResult {
        map,
        size,
        start_point,
    } = parse_input(input);

    let mut previous_position = start_point;
    let mut current_position = start_point;

    let mut loop_map = Array2::from_elem(size.into_tuple(), '.');
    loop_map[start_point.into_tuple()] = 'S'; // TODO could be wrong for other inputs?

    loop {
        let next_position = find_next_pipe(&map, current_position, previous_position).unwrap();

        if next_position == start_point {
            break;
        }

        loop_map[next_position.into_tuple()] = map[next_position.into_tuple()];
        previous_position = current_position;
        current_position = next_position;
    }

    let mut enclosed_count = 0;

    for x in 0..size.x {
        let mut pipe_count = 0;

        #[derive(PartialEq)]
        enum Turn {
            Left,
            Right,
        }

        // Default value, will always be changed before being used
        let mut previous_turn = Turn::Left;

        for y in 0..size.y {
            let pipe = loop_map[(x, y)];

            match pipe {
                '═' => {
                    pipe_count += 1;
                }

                // pipe if pipe == '╗' || pipe == '╔' => {}
                '╗' => {
                    previous_turn = Turn::Left;
                }

                '╔' => previous_turn = Turn::Right,

                '╝' => {
                    if previous_turn == Turn::Left {
                        pipe_count += 2;
                    } else {
                        pipe_count += 1;
                    }
                }

                '╚' => {
                    if previous_turn == Turn::Right {
                        pipe_count += 2;
                    } else {
                        pipe_count += 1;
                    }
                }

                '.' => {
                    if pipe_count % 2 != 0 {
                        enclosed_count += 1;
                    }
                }

                _ => {}
            }
        }
    }

    enclosed_count
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
        assert_eq!(result, 6956);
    }

    #[test]
    fn test_part2() {
        let result = part_2(INPUT);
        assert_eq!(result, 455);
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
