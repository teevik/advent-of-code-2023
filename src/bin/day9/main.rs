#![feature(test)]
#![feature(iter_map_windows)]

use itertools::Itertools;
use std::rc::Rc;

fn parse_history_line(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|value| value.parse::<i32>().unwrap())
        .collect_vec()
}

fn parse_input(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(parse_history_line)
}

fn calculate_history_diffs(history: Vec<i32>) -> Vec<Rc<Vec<i32>>> {
    let history = Rc::new(history);

    let mut all_differences = vec![history.clone()];
    let mut history_differences = history;

    while history_differences.iter().any(|&value| value != 0) {
        history_differences = history_differences
            .iter()
            .copied()
            .map_windows(|&[left, right]| right - left)
            .collect_vec()
            .into();

        all_differences.push(history_differences.clone());
    }

    all_differences
}

fn part_1(input: &str) -> i32 {
    let histories = parse_input(input);

    let predicted_values = histories.map(|history| {
        let all_differences = calculate_history_diffs(history);

        let predicted_value = all_differences
            .into_iter()
            .map(|values| values.last().copied().unwrap_or(0))
            .sum::<i32>();

        predicted_value
    });

    predicted_values.sum()
}

fn part_2(input: &str) -> i32 {
    let histories = parse_input(input);

    let predicted_values = histories.map(|history| {
        let all_differences = calculate_history_diffs(history);

        let predicted_value = all_differences
            .into_iter()
            .map(|values| values.first().copied().unwrap_or(0))
            .rfold(0, |acc, item| item - acc);

        predicted_value
    });

    predicted_values.sum()
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
        assert_eq!(result, 2098530125);
    }

    #[test]
    fn test_part2() {
        let result = part_2(INPUT);
        assert_eq!(result, 1016);
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
