#![feature(test)]

use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

enum Instruction {
    Left,
    Right,
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

struct Network<'a> {
    instructions: Vec<Instruction>,
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Network<'a> {
    fn amount_of_steps(&self, from: &'a str, to: impl Fn(&'a str) -> bool) -> u64 {
        let mut steps: u64 = 0;
        let mut position = from;

        loop {
            for instruction in &self.instructions {
                let node = self.nodes.get(position).unwrap();

                let next_position = match instruction {
                    Instruction::Left => node.left,
                    Instruction::Right => node.right,
                };

                position = next_position;
                steps += 1;

                if to(position) {
                    return steps;
                }
            }
        }
    }
}

fn parse_network(input: &str) -> Network<'_> {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let instructions = instructions
        .as_bytes()
        .iter()
        .map(|instruction| match instruction {
            b'L' => Instruction::Left,
            b'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let nodes = nodes
        .lines()
        .map(|line| {
            let from = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (from, Node { left, right })
        })
        .collect::<HashMap<_, _>>();

    Network {
        instructions,
        nodes,
    }
}

fn part_1(input: &str) -> u64 {
    let network = parse_network(input);

    network.amount_of_steps("AAA", |node| node == "ZZZ")
}

fn part_2(input: &str) -> u64 {
    let network = parse_network(input);

    let start_positions = network
        .nodes
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'));

    let steps_for_each_cycle =
        start_positions.map(|from| network.amount_of_steps(from, |node| node.ends_with('Z')));

    return steps_for_each_cycle.fold(1, |acc, steps| lcm(acc, steps));
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
        assert_eq!(result, 16579);
    }

    #[test]
    fn test_part2() {
        let result = part_2(INPUT);
        assert_eq!(result, 12927600769609);
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
