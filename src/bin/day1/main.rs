#![feature(test)]

struct CalibrationDigits {
    first_digit: u64,
    last_digit: u64,
}

fn parse_part1(input: &str) -> impl Iterator<Item = CalibrationDigits> + '_ {
    input.lines().map(|line| {
        let first_digit = line
            .chars()
            .find_map(|char| char.to_digit(10))
            .expect("Always there") as u64;

        let last_digit = line
            .chars()
            .rev()
            .find_map(|char| char.to_digit(10))
            .expect("Always there") as u64;

        CalibrationDigits {
            first_digit,
            last_digit,
        }
    })
}

fn parse_part2(input: &str) -> impl Iterator<Item = CalibrationDigits> + '_ {
    fn find_word_digit(word: &str) -> Option<u64> {
        // one, two, three, four, five, siz, seven, eight, nine
        // max length 5

        let length = word.len();

        if length < 3 {
            return None;
        }

        match &word[0..3] {
            "one" => return Some(1),
            "two" => return Some(2),
            "six" => return Some(6),
            _ => {}
        }

        if length < 4 {
            return None;
        }

        match &word[0..4] {
            "four" => return Some(4),
            "five" => return Some(5),
            "nine" => return Some(9),
            _ => {}
        }

        if length < 5 {
            return None;
        }

        match &word[0..5] {
            "three" => return Some(3),
            "seven" => return Some(7),
            "eight" => return Some(8),
            _ => {}
        }

        None
    }

    input.lines().map(|line| {
        let find_at_index = |index: usize| {
            if let Some(digit) = line.chars().nth(index).unwrap().to_digit(10) {
                return Some(digit as u64);
            }

            if let Some(digit) = find_word_digit(&line[index..]) {
                return Some(digit);
            }

            None
        };

        let first_digit = (0..line.len())
            .find_map(find_at_index)
            .expect("Always there");

        let last_digit = (0..line.len())
            .rev()
            .find_map(find_at_index)
            .expect("Always there");

        CalibrationDigits {
            first_digit,
            last_digit,
        }
    })
}

fn calculate(digits: impl Iterator<Item = CalibrationDigits>) -> u64 {
    digits
        .map(
            |CalibrationDigits {
                 first_digit,
                 last_digit,
             }| first_digit * 10 + last_digit,
        )
        .sum()
}

fn main() {
    let input = include_str!("input.txt");

    let part_1 = calculate(parse_part1(input));
    let part_2 = calculate(parse_part2(input));

    dbg!(part_1);
    dbg!(part_2);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[bench]
    fn bench_parse_part1(b: &mut test::Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            for parsed in parse_part1(input) {
                test::black_box(parsed);
            }
        });
    }

    #[bench]
    fn bench_parse_part2(b: &mut test::Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            for parsed in parse_part2(input) {
                test::black_box(parsed);
            }
        });
    }

    #[bench]
    fn bench_calculate(b: &mut test::Bencher) {
        let input = include_str!("input.txt");

        b.iter(|| {
            let result = calculate(parse_part1(input));
            test::black_box(result);
        });
    }
}
